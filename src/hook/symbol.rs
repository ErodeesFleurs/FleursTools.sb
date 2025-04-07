use std::collections::HashMap;

#[cfg(target_os = "linux")]
mod linux {
    use super::*;

    use std::fs;
    use std::process;

    use goblin::elf;

    pub(crate) fn parse_symbols() -> anyhow::Result<HashMap<String, u64>> {
        let pid = process::id();
        let exe_path = format!("/proc/{}/exe", pid);

        let buffer = fs::read(exe_path)?;

        let elf = elf::Elf::parse(&buffer)?;

        let mut symbols_map = HashMap::new();

        for sym in elf.syms.iter() {
            if let Some(name) = elf.strtab.get_at(sym.st_name) {
                if sym.is_function() {
                    symbols_map.insert(name.to_string(), sym.st_value);
                }
            }
        }

        Ok(symbols_map)
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use super::*;
    
    use std::fs;

    use pdb::{FallibleIterator, SymbolData};

    pub(crate) fn parse_symbols(path: &str) -> anyhow::Result<HashMap<String, u64>> {
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
}

#[cfg(target_os = "linux")]
pub fn platform_parse_symbols(_: &str) -> anyhow::Result<HashMap<String, u64>> {
    linux::parse_symbols()
}

#[cfg(target_os = "windows")]
pub fn platform_parse_symbols(path: &str) -> anyhow::Result<HashMap<String, u64>> {
    windows::parse_symbols(path)
}