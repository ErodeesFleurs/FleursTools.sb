use std::io::{Read, Write};

use anyhow::Ok;

use::byteorder::ReadBytesExt;

#[derive(Debug)]
pub struct VLQu64(pub u64);

#[allow(dead_code)]
impl VLQu64 {
    pub const MAX_SIZE: usize = 10; // Maximum size of a VLQ encoded value

    pub fn encode<W: Write>(&self, output: &mut W) -> anyhow::Result<usize> {
        let mut buf = Vec::new();
        let mut val = self.0;

        buf.push((val & 127) as u8);
        val >>= 7;

        while val != 0 {
            buf.push((val & 127 | 128) as u8);
            val >>= 7;
        }

        buf.reverse();
        output.write_all(&buf)?;
        
        Ok(buf.len())
    }

    pub fn decode<R: Read + Sized>(input: &mut R) -> anyhow::Result<Self> {
        let mut val: u64 = 0;
        for _ in 0..Self::MAX_SIZE {
            let byte: u8 = input.read_u8()?;
            val = val << 7 | (byte & 127) as u64;
            if byte & 128 == 0 {
                return Ok(VLQu64(val));
            }
        }
        anyhow::bail!("OversizedVLQ")
    }
}

#[derive(Debug)]
pub struct VLQi64(pub i64);

#[allow(dead_code)]
impl VLQi64 {
    pub const MAX_SIZE: usize = 10; // Maximum size of a VLQ encoded value

    pub fn encode<W: Write>(&self, output: &mut W) -> anyhow::Result<usize> {
        let target;
        if self.0 < 0 {
            target = (((-(self.0 + 1)) << 1) | 1) as u64;
        } else {
            target = (self.0 << 1) as u64;
        }
        Ok(VLQu64::encode(&VLQu64(target), output)?)
    }

    pub fn decode<R: Read>(input: &mut R) -> anyhow::Result<Self> {
        let VLQu64(value) = VLQu64::decode(input)?;
        if value & 1 == 0 {
            Ok(VLQi64((value >> 1) as i64))
        } else {
            Ok(VLQi64(-((value >> 1) as i64) - 1))
        }
    }
}
