use std::collections::HashMap;

use mlua::IntoLua;

mod vlq;

pub mod writer;
pub mod reader;

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