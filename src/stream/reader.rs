use std::collections::HashMap;
use std::io::Read;

use anyhow::Ok;
use byteorder::{BigEndian, ReadBytesExt};

use super::SBType;
use super::vlq::{VLQi64, VLQu64};

pub trait SBReader: Read + Sized {
    fn read_vlq_u64(&mut self) -> anyhow::Result<u64> {
        Ok(VLQu64::decode(self)?.0)
    }

    fn read_vlq_i64(&mut self) -> anyhow::Result<i64> {
        Ok(VLQi64::decode(self)?.0)
    }

    fn read_bytes(&mut self) -> anyhow::Result<Vec<u8>> {
        let length = self.read_vlq_u64()?;
        let mut buffer = vec![0u8; length as usize];
        self.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    fn read_string(&mut self) -> anyhow::Result<String> {
        let bytes = self.read_bytes()?;
        Ok(String::from_utf8(bytes)?)
    }

    fn read_array(&mut self) -> anyhow::Result<Vec<SBType>> {
        let mut array = Vec::new();
        let length = self.read_vlq_u64()?;

        for _ in 0..length {
            array.push(self.read_object()?);
        }

        Ok(array)
    }

    fn read_map(&mut self) -> anyhow::Result<HashMap<String, SBType>> {
        let mut map = HashMap::new();
        let length = self.read_vlq_u64()?;

        for _ in 0..length {
            map.insert(self.read_string()?, self.read_object()?);
        }

        Ok(map)
    }

    fn read_object(&mut self) -> anyhow::Result<SBType> {
        let index = self.read_u8()? - 1;

        Ok(match index {
            0 => SBType::Nil,
            1 => SBType::Float(self.read_f64::<BigEndian>()?),
            2 => SBType::Boolean(self.read_u8()? != 0),
            3 => SBType::Int(self.read_vlq_i64()?),
            4 => SBType::String(self.read_string()?),
            5 => SBType::Array(self.read_array()?),
            6 => SBType::Object(self.read_map()?),
            _ => anyhow::bail!("Unsupport type"),
        })
    }
}

impl SBReader for std::io::Cursor<Vec<u8>> {}