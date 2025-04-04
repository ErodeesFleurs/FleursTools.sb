use std::{collections::HashMap, io::Cursor};

use file::AssetFile;

use crate::stream::SBType;

mod packet;
mod file; 

trait AssetReader {
    fn file(&mut self, path: &str) -> anyhow::Result<AssetFile>;

    fn exist(&self, path: &str) -> bool;

    fn paths(&self) -> Vec<&String>;

    fn meta(&self, key: String) -> anyhow::Result<SBType>;
}

pub fn register_asset(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let asset: mlua::Table = lua.create_table()?;
    let packet_reader: mlua::Function = lua.create_function(|_, path: String| {
        let cursor = Cursor::new(std::fs::read(path)?);
        Ok(packet::create_packet_reader(cursor))
    })?;
    asset.set("PacketReader", packet_reader)?;
    Ok(asset)
}
