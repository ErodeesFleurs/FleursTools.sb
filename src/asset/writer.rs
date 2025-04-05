use std::collections::HashMap;
use std::io::Write;

use byteorder::{BigEndian, WriteBytesExt};

use super::SBType;
use super::vlq::{VLQi64, VLQu64};

#[allow(unused)]
pub trait SBWriter: Write {
    fn write_vlq_u64(&mut self, value: u64) -> anyhow::Result<()> {
        let mut buffer = Vec::new();
        VLQu64(value).encode(&mut buffer)?;
        self.write_all(&buffer)?;
        Ok(())
    }

    fn write_vlq_i64(&mut self, value: i64) -> anyhow::Result<()> {
        let mut buffer = Vec::new();
        VLQi64(value).encode(&mut buffer)?;
        self.write_all(&buffer)?;
        Ok(())
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> anyhow::Result<()> {
        let length = bytes.len() as u64;
        self.write_vlq_u64(length)?;
        self.write_all(bytes)?;
        Ok(())
    }

    fn write_string(&mut self, string: &str) -> anyhow::Result<()> {
        let bytes = string.as_bytes();
        self.write_bytes(bytes)
    }

    fn write_array(&mut self, array: Vec<SBType>) -> anyhow::Result<()> {
        let length = array.len() as u64;
        self.write_vlq_u64(length)?;
        for value in array {
            self.write_object(&value)?;
        }
        Ok(())
    }

    fn write_map(&mut self, map: HashMap<String, SBType>) -> anyhow::Result<()> {
        let length = map.len() as u64;
        self.write_vlq_u64(length)?;
        for (key, value) in map {
            self.write_string(&key)?;
            self.write_object(&value)?;
        }
        Ok(())
    }

    fn write_object(&mut self, object: &SBType) -> anyhow::Result<()> {
        self.write_u8(u8::try_from(object.clone())? + 1)?;
        match object {
            SBType::Nil => {}
            SBType::Float(value) => self.write_f64::<BigEndian>(*value)?,
            SBType::Boolean(value) => self.write_u8(if *value { 1 } else { 0 })?,
            SBType::Int(value) => self.write_vlq_i64(*value)?,
            SBType::String(value) => self.write_string(value)?,
            SBType::Array(array) => self.write_array(array.clone())?,
            SBType::Object(object) => self.write_map(object.clone())?,
        };
        Ok(())
    }
}
