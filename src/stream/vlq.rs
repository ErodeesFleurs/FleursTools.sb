use std::io::{Read, Write};

use anyhow::Ok;
pub struct VLQu64(u64);

impl VLQu64 {
    pub const MAX_SIZE: usize = 10; // Maximum size of a VLQ encoded value

    fn encode<W: Write>(&self, output: &mut W) -> anyhow::Result<usize>
    where
        W: Write,
    {
        let mut value = self.0;
        let mut bytes_written = 0;

        while value > 127 {
            output.write_all(&[(value as u8) | 128])?;
            value >>= 7;
            bytes_written += 1;
        }
        output.write_all(&[value as u8])?;
        bytes_written += 1;

        Ok(bytes_written)
    }

    fn decode<R: Read>(input: &mut R) -> anyhow::Result<Self> {
        let mut value = 0u64;
        let mut shift = 0;
        let mut bytes_read = 0;

        loop {
            if bytes_read >= Self::MAX_SIZE {
                anyhow::bail!("VLQ value too large");
            }

            let mut buffer = [0u8; 1];
            input.read_exact(&mut buffer)?;
            let byte = buffer[0];
            value |= ((byte & 127) as u64) << shift;
            bytes_read += 1;

            if byte & 128 == 0 {
                break;
            }
            shift += 7;
        }

        Ok(VLQu64(value))
    }
}

pub struct VLQi64(i64);

impl VLQi64 {
    pub const MAX_SIZE: usize = 10; // Maximum size of a VLQ encoded value

    fn encode<W: Write>(&self, output: &mut W) -> anyhow::Result<usize> {
        let target;
        if self.0 < 0 {
            target = (((-(self.0 + 1)) << 1) | 1) as u64;
        } else {
            target = (self.0 << 1) as u64;
        }
        Ok(VLQu64::encode(&VLQu64(target), output)?)
    }

    fn decode<R: Read>(input: &mut R) -> anyhow::Result<Self> {
        let VLQu64(value) = VLQu64::decode(input)?;
        if value & 1 == 0 {
            Ok(VLQi64((value >> 1) as i64))
        } else {
            Ok(VLQi64(-((value >> 1) as i64) - 1))
        }
    }
}
