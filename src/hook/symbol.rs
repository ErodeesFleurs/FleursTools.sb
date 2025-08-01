use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

static GLOBAL_SYMBOLS: LazyLock<Mutex<HashMap<String, u64>>> =
    LazyLock::new(|| HashMap::new().into());

pub const PROGRAM_NAME: &str = "openstarbound";

#[cfg(target_os = "linux")]
mod linux {
    use super::*;

    use std::{fs, ffi};

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

    pub fn dynamic_symbol_addr(_: &str, name: &str) -> anyhow::Result<u64> {
        let c_name = ffi::CString::new(name).map_err(|_| anyhow::anyhow!("CString error"))?;

        let addr = unsafe { libc::dlsym(libc::RTLD_DEFAULT, c_name.as_ptr()) };

        if addr.is_null() {
            anyhow::bail!("Symbol not found: {}", name);
        } else {
            Ok(addr as u64)
        }
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use super::*;

    use std::{fs, ffi};

    use pdb::FallibleIterator;

    use tklog::{debug, handle};
    use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};

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
    
        let symbol_table = pdb.global_symbols()?;

        let address_map = pdb.address_map()?;

        let mut symbols = symbol_table.iter();

        while let Some(symbol) = symbols.next().unwrap() {
            if let pdb::SymbolData::Public(data) = symbol.parse().unwrap() {
                let name = data.name.to_string().into_owned();
                let name = demangle_msvc_function_name(&name);
                
                let Some(offset) = data.offset.to_rva(&address_map) else {
                    continue;
                };

                symbols_map.insert(name, offset.0 as u64);
            }
        }

        Ok(symbols_map.clone())
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

    pub fn dynamic_symbol_addr(module: &str, name: &str) -> anyhow::Result<u64> {
        let c_name = ffi::CString::new(name).map_err(|_| anyhow::anyhow!("CString error"))?;
        let c_module = ffi::CString::new(module).map_err(|_| anyhow::anyhow!("CString error"))?;

        let handle = unsafe { GetModuleHandleA(c_module.as_ptr()) };

        if handle.is_null() {
            anyhow::bail!("Module not found: {}", module);
        }

        let addr = unsafe {
            GetProcAddress(
                handle,
                c_name.as_ptr(),
            )
        };

        if addr.is_null() {
            anyhow::bail!("Symbol not found: {}", name);
        } else {
            Ok(addr as u64)
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

#[cfg(target_os = "linux")]
pub fn platform_dynamic_symbol_addr(module: &str, name: &str) -> anyhow::Result<u64> {
    linux::dynamic_symbol_addr(module, name)
}
#[cfg(target_os = "windows")]
pub fn platform_dynamic_symbol_addr(module: &str, name: &str) -> anyhow::Result<u64> {
    windows::dynamic_symbol_addr(module, name)
}


pub fn symbol_addr(name: &str) -> anyhow::Result<Option<u64>> {
    let symbols_map = GLOBAL_SYMBOLS
        .lock()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    Ok(symbols_map
        .iter()
        .find_map(|(k, &v)| if k.starts_with(name) { Some(v) } else { None }))
}

pub fn register_function(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let symbols_table = lua.create_table()?;

    let init_symbols = lua.create_function(
        |lua: &mlua::Lua, path: Option<String>| -> mlua::Result<mlua::Table> {
            let symbols =
                platform_parse_symbols(path).map_err(|e| mlua::Error::external(e))?;

            let symbols_table = lua.create_table()?;

            for (name, addr) in symbols {
                symbols_table.set(name, addr)?;
            }

            Ok(symbols_table)
        },
    )?;
    symbols_table.set("init_symbols", init_symbols)?;

    let base_addr = lua.create_function(|_, name: String| -> mlua::Result<u64> {
        let addr = platform_base_addr(&name);
        addr.map_err(|e| mlua::Error::external(e))
    })?;
    symbols_table.set("base_addr", base_addr)?;

    let symbol_addr = lua.create_function(|_, name: String| -> mlua::Result<Option<u64>> {
        let addr = symbol_addr(&name);
        addr.map_err(|e| mlua::Error::external(e))
    })?;
    symbols_table.set("symbol_addr", symbol_addr)?;

    let dynamic_symbol_addr =
        lua.create_function(|_, (module, name): (String, String)| -> mlua::Result<u64> {
            let addr = platform_dynamic_symbol_addr(&module, &name);
            addr.map_err(|e| mlua::Error::external(e))
        })?;
    symbols_table.set("dynamic_symbol_addr", dynamic_symbol_addr)?;

    Ok(symbols_table)
}

// #[cfg(test)]
// mod tests {
//     use std::fs::File;

//     use pdb::FallibleIterator;
//     use tklog::debug;

//     use crate::log;

//     #[test]
//     fn test_read_pdb() {
//         log::log_init();
//         let path = "/media/next/SteamLibrary/steamapps/common/Starbound/win64/starbound.pdb";
//         let file = File::open(path).unwrap();
//         let mut pdb = pdb::PDB::open(file).unwrap();
//         let symbol_table = pdb.global_symbols().unwrap();

//         let address_map = pdb.address_map().unwrap();

//         let mut symbols = symbol_table.iter();

//         while let Some(symbol) = symbols.next().unwrap() {
//             if let pdb::SymbolData::Public(data) = symbol.parse().unwrap() {
//                 let name = data.name.to_string().into_owned();
//                 // 输出符号名 + RVA（相对虚拟地址）
//                 let Some(rva) = data.offset.to_rva(&address_map) else {
//                     continue;
//                 };
//                 debug!("Symbol: {} RVA: {:#x}", name, rva.0 as u64);
//             }
//         }
//     }
// }
