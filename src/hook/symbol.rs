use std::collections::HashMap;

#[cfg(target_os = "linux")]
mod linux {
    use super::*;

    use std::fs;

    use goblin::elf;

    fn demangle_function_name(name: &str) -> String {
        cpp_demangle::Symbol::new(name)
            .map(|symbol| symbol.to_string())
            .unwrap_or_else(|_| name.to_string())
    }

    pub fn parse_symbols(_: &str) -> anyhow::Result<HashMap<String, u64>> {
        let buffer = fs::read("/proc/self/exe")?;

        let elf = elf::Elf::parse(&buffer)?;

        let mut symbols_map = HashMap::new();

        for sym in elf.syms.iter() {
            if let Some(name) = elf.strtab.get_at(sym.st_name) {
                if sym.is_function() {
                    let name = demangle_function_name(name);
                    symbols_map.insert(name.to_string(), sym.st_value);
                }
            }
        }

        Ok(symbols_map)
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

    use winapi::um::libloaderapi::GetModuleHandleA;

    pub fn open_pdb(path: &str) -> anyhow::Result<bool> {
        let file = fs::File::open(path)?;

        let mut pdb = pdb::PDB::open(file)?;

        let symbol_table = pdb.global_symbols()?;

        return true;
    }

    pub fn parse_symbols(path: &str) -> anyhow::Result<HashMap<String, u64>> {
        let file = fs::File::open(path)?;

        let mut pdb = pdb::PDB::open(file)?;

        let symbol_table = pdb.global_symbols()?;

        let mut symbols = HashMap::new();

        while let Some(symbol) = symbol_table.iter().next()? {
            match symbol.parse()? {
                SymbolData::Public(data) => {
                    symbols.insert(
                        data.name.to_string().into_owned(),
                        data.offset.offset as u64,
                    );
                }
                SymbolData::Procedure(data) => {
                    symbols.insert(
                        data.name.to_string().into_owned(),
                        data.offset.offset as u64,
                    );
                }
                _ => {}
            }
        }

        Ok(symbols)
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


#[cfg(target_os = "windows")]
pub fn open_pdb(path: &str) -> anyhow::Result<bool> {
    windows::open_pdb(path)
}