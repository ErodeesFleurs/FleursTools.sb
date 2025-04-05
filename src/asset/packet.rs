use std::collections::HashMap;
use std::io::{Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use super::SBType;
use super::AssetReader;
use super::file::AssetFile;
use super::reader::SBReader;

const ASSET_HEADER: [u8; 8] = *b"SBAsset6";
const INDEX_HEADER: [u8; 5] = *b"INDEX";

#[derive(Debug)]
pub struct PacketReader<R>
where R: SBReader + Seek
{
    // (offset, length)
    index: HashMap<String, (u64, u64)>,
    metadata: HashMap<String, SBType>,
    buffer: R
}

impl<R> PacketReader<R>
where R: SBReader + Seek
{
    pub fn new(mut input: R) -> anyhow::Result<Self> {
        let mut magic = [0u8; 8];

        input.read_exact(&mut magic)?;

        if magic != ASSET_HEADER {
            anyhow::bail!("Invalid packed file header");
        }

        let index_start = input.read_u64::<BigEndian>()?;
        input.seek(SeekFrom::Start(index_start))?;

        let mut header = [0u8; 5];
        input.read_exact(&mut header)?;

        if header != INDEX_HEADER {
            anyhow::bail!("Invalid index header");
        }

        let metadata = input.read_map()?;
        let mut index = HashMap::new();

        for _ in 0..input.read_vlq_u64()? {
            index.insert(
                input.read_string()?,
                (
                    input.read_u64::<BigEndian>()?,
                    input.read_u64::<BigEndian>()?,
                ),
            );
        }

        Ok(Self { index, metadata, buffer: input })
    }
}

impl<R> AssetReader for PacketReader<R>
where R: SBReader + Seek
{
    fn exist(&self, path: &str) -> bool {
        self.index.contains_key(path)
    }
    
    fn file(&mut self, path: &str) -> anyhow::Result<AssetFile> {
        if !self.exist(path) {
            anyhow::bail!("File is not exist")
        }
        let path = path.to_string();

        let info = self.index[&path];

        let mut bytes = vec![0u8; info.1 as usize];
        self.buffer.seek(SeekFrom::Start(info.0))?;

        self.buffer.read_exact(&mut bytes)?;

        Ok(AssetFile { path, bytes })
    }
    
    fn paths(&self) -> Vec<&String> {
        self.index.keys().collect()
    }

    fn meta(&self, key: String) -> anyhow::Result<SBType> {
        Ok(self.metadata[&key].clone())
    }
}