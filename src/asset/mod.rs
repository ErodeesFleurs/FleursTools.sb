mod directory;
mod file;
mod packet;
mod reader;
mod vlq;
mod writer;

use std::collections::HashMap;
use std::io::Cursor;

use mlua::IntoLua;

use file::AssetFile;
use packet::PacketReader;

#[derive(Debug, Clone)]
pub enum SBType {
    Nil,
    Float(f64),
    Boolean(bool),
    Int(i64),
    String(String),
    Array(Vec<SBType>),
    Object(HashMap<String, SBType>),
}

impl TryFrom<SBType> for u8 {
    type Error = anyhow::Error;

    fn try_from(value: SBType) -> Result<Self, Self::Error> {
        match value {
            SBType::Nil => Ok(0),
            SBType::Float(_) => Ok(1),
            SBType::Boolean(_) => Ok(2),
            SBType::Int(_) => Ok(3),
            SBType::String(_) => Ok(4),
            SBType::Array(_) => Ok(5),
            SBType::Object(_) => Ok(6),
        }
    }
}

impl TryFrom<json::JsonValue> for SBType {
    type Error = anyhow::Error;

    fn try_from(value: json::JsonValue) -> Result<Self, Self::Error> {
        match  value {
            json::JsonValue::Null => Ok(SBType::Nil),
            json::JsonValue::Short(short) => Ok(SBType::String(short.to_string())),
            json::JsonValue::String(string) => Ok(SBType::String(string)),
            json::JsonValue::Number(number) => {
                let num: f64  = number.into();
                if num.fract() == 0.0 {
                    Ok(SBType::Int(num as i64))
                } else {
                    Ok(SBType::Float(num))
                }
            }
            json::JsonValue::Boolean(boolean) => {
                Ok(SBType::Boolean(boolean))
            },
            json::JsonValue::Object(object) => {
                let mut map = HashMap::new();
                for (key, value) in object.iter() {
                    map.insert(key.to_string(), SBType::try_from(value.clone())?);
                }
                Ok(SBType::Object(map))
            }
            json::JsonValue::Array(json_values) => {
                let mut array = Vec::new();
                for value in json_values {
                    array.push(SBType::try_from(value)?);
                }
                Ok(SBType::Array(array))
            },
        }
    }
}

impl IntoLua for SBType {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        match self {
            SBType::Nil => Ok(mlua::Value::Nil),
            SBType::Float(v) => Ok(mlua::Value::Number(v)),
            SBType::Boolean(v) => Ok(mlua::Value::Boolean(v)),
            SBType::Int(v) => Ok(mlua::Value::Integer(v)),
            SBType::String(v) => Ok(mlua::Value::String(lua.create_string(&v)?)),
            SBType::Array(v) => {
                let table = lua.create_table()?;
                for (i, item) in v.into_iter().enumerate() {
                    table.set(i + 1, item)?;
                }
                Ok(mlua::Value::Table(table))
            }
            SBType::Object(v) => {
                let table = lua.create_table()?;
                for (k, v) in v {
                    table.set(k, v)?;
                }
                Ok(mlua::Value::Table(table))
            }
        }
    }
}

trait AssetReader {
    fn file(&mut self, path: &str) -> anyhow::Result<AssetFile>;

    fn exist(&self, path: &str) -> bool;

    fn paths(&self) -> Vec<&String>;

    fn meta(&self, key: String) -> anyhow::Result<SBType>;
}

enum AssetReaderEnum {
    PacketReader(PacketReader<Cursor<Vec<u8>>>),
    DirectoryReader(directory::DirectoryReader),
}

impl mlua::UserData for AssetReaderEnum {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("file", |_, this, path: String| {
            match this {
                AssetReaderEnum::PacketReader(reader) => reader.file(&path),
                AssetReaderEnum::DirectoryReader(reader) => reader.file(&path),
            }
            .map_err(|e| e.into())
        });

        methods.add_method("exist", |_, this, path: String| match this {
            AssetReaderEnum::PacketReader(reader) => Ok(reader.exist(&path)),
            AssetReaderEnum::DirectoryReader(reader) => Ok(reader.exist(&path)),
        });

        methods.add_method("paths", |_, this, _: ()| match this {
            AssetReaderEnum::PacketReader(reader) => {
                Ok(reader.paths().into_iter().cloned().collect::<Vec<String>>())
            }
            AssetReaderEnum::DirectoryReader(reader) => {
                Ok(reader.paths().into_iter().cloned().collect::<Vec<String>>())
            }
        });

        methods.add_method("meta", |_, this, key: String| {
            match this {
                AssetReaderEnum::PacketReader(reader) => reader.meta(key),
                AssetReaderEnum::DirectoryReader(reader) => reader.meta(key),
            }
            .map_err(|e| e.into())
        });
    }
}

pub fn register_asset(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let asset = lua.create_table()?;

    let asset_reader = lua.create_function(|_, path: String| -> mlua::Result<AssetReaderEnum> {
        if path.ends_with(".pak") {
            let cursor = Cursor::new(std::fs::read(path)?);
            let packet_reader = PacketReader::new(cursor);
            Ok(AssetReaderEnum::PacketReader(packet_reader?))
        } else {
            let directory_reader =
                directory::DirectoryReader::new(&path);
            Ok(AssetReaderEnum::DirectoryReader(directory_reader?))
        }
    })?;

    asset.set("AssetReader", asset_reader)?;

    Ok(asset)
}
