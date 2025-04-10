use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

static GLOBAL_SYMBOLS: LazyLock<Mutex<HashMap<String, u64>>> =
    LazyLock::new(|| HashMap::new().into());

#[cfg(target_os = "linux")]
mod linux {
    use super::*;

    use std::fs;

    use goblin::elf;

    pub fn demangle_itanium_function_name(name: &str) -> String {
        cpp_demangle::Symbol::new(name)
            .map(|symbol| symbol.to_string())
            .unwrap_or_else(|_| name.to_string())
    }

    pub fn parse_symbols(_: &str) -> anyhow::Result<HashMap<String, u64>> {
        let mut symbols_map = GLOBAL_SYMBOLS
            .lock()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        if symbols_map.len() > 0 {
            return Ok(symbols_map.clone());
        }

        let buffer = fs::read("/proc/self/exe")?;

        let elf = elf::Elf::parse(&buffer)?;

        for sym in elf.syms.iter() {
            if let Some(name) = elf.strtab.get_at(sym.st_name) {
                if sym.is_function() {
                    let name = demangle_itanium_function_name(name);
                    symbols_map.insert(name.to_string(), sym.st_value);
                }
            }
        }

        Ok(symbols_map.clone())
    }

    pub fn base_addr(name: &str) -> anyhow::Result<u64> {
        let buffer = fs::read("/proc/self/maps")?;

        let maps = String::from_utf8(buffer)?;

        for line in maps.lines() {
            if line.contains(name) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(addr) = parts.get(0) {
                    let addr_parts: Vec<&str> = addr.split('-').collect();
                    if let Some(start_addr) = addr_parts.get(0) {
                        return Ok(u64::from_str_radix(start_addr, 16)?);
                    }
                }
            }
        }
        anyhow::bail!("Base address not found for {}", name)
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use super::*;

    use std::fs;

    use pdb::{FallibleIterator, SymbolData};

    use tklog::debug;
    use winapi::um::libloaderapi::GetModuleHandleA;

    fn demangle_msvc_function_name(name: &str) -> String {
        if !name.starts_with('?') {
            return name.to_string();
        }

        let parts = name
            .trim_start_matches('?')
            .split('@')
            .collect::<Vec<&str>>();
        let function_name = parts.get(0).unwrap_or(&name);

        let namespace = parts
            .iter()
            .skip(1)
            .take_while(|&&s| s != "")
            .map(|&s| s)
            .collect::<Vec<&str>>()
            .into_iter()
            .rev()
            .collect::<Vec<&str>>();

        format!("{}::{}", namespace.join("::"), function_name)
    }

    pub fn parse_symbols(path: &str) -> anyhow::Result<HashMap<String, u64>> {
        let mut symbols_map = GLOBAL_SYMBOLS
            .lock()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        if symbols_map.len() > 0 {
            return Ok(symbols_map.clone());
        }

        let file = fs::File::open(path)?;

        let mut pdb = pdb::PDB::open(file)?;

        debug!("Parsing PDB file: {}", path);

        let symbol_table = pdb.global_symbols()?;

        debug!("Parsing symbols from PDB file");

        let mut count = 0;

        while let Some(symbol) = symbol_table.iter().next()? {
            match symbol.parse()? {
                SymbolData::Public(data) => {
                    symbols_map.insert(
                        demangle_msvc_function_name(&data.name.to_string().into_owned()),
                        data.offset.offset as u64,
                    );
                }
                SymbolData::Procedure(data) => {
                    symbols_map.insert(
                        demangle_msvc_function_name(&data.name.to_string().into_owned()),
                        data.offset.offset as u64,
                    );
                }
                _ => {}
            }
            count += 1;
            if count % 100 == 0 {
                debug!("Parsed {} symbols", count);
            }
        }

        debug!("Parsed {} symbols", symbols_map.len());

        Ok(symbols_map.clone())
    }

    pub fn symbol_addr(name: &str) -> anyhow::Result<u64> {
        let file = fs::File::open(name)?;

        let mut pdb = pdb::PDB::open(file)?;

        let symbol_table = pdb.global_symbols()?;

        while let Some(symbol) = symbol_table.iter().next()? {
            match symbol.parse()? {
                SymbolData::Public(data) => {
                    if data.name.to_string().into_owned() == name.to_string() {
                        return Ok(data.offset.offset as u64);
                    }
                }
                SymbolData::Procedure(data) => {
                    if data.name.to_string().into_owned() == name.to_string() {
                        return Ok(data.offset.offset as u64);
                    }
                }
                _ => {}
            }
        }

        anyhow::bail!("Symbol {} not found", name)
    }

    pub fn base_addr() -> anyhow::Result<u64> {
        // 获取当前进程的基地址
        let handle = unsafe { GetModuleHandleA(std::ptr::null()) };
        if handle.is_null() {
            anyhow::bail!("Failed to get module handle");
        } else {
            Ok(handle as u64) // Cast to u64 instead of usize
        }
    }
}

#[cfg(target_os = "linux")]
pub fn platform_parse_symbols(_: Option<String>) -> anyhow::Result<HashMap<String, u64>> {
    linux::parse_symbols("")
}

#[cfg(target_os = "windows")]
pub fn platform_parse_symbols(path: Option<String>) -> anyhow::Result<HashMap<String, u64>> {
    if let Some(pdb_path) = path {
        return windows::parse_symbols(&pdb_path);
    }
    anyhow::bail!("Cant find PDB file")
}

#[cfg(target_os = "linux")]
pub fn platform_base_addr(name: &str) -> anyhow::Result<u64> {
    linux::base_addr(name)
}
#[cfg(target_os = "windows")]
pub fn platform_base_addr(_: &str) -> anyhow::Result<u64> {
    windows::base_addr()
}

pub fn symbol_addr(name: &str) -> anyhow::Result<Option<u64>> {
    let symbols_map = GLOBAL_SYMBOLS
        .lock()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    Ok(symbols_map.iter().find_map(|(k, &v)| {
        if k.starts_with(name) {
            Some(v)
        } else {
            None
        }
    }))
}