use std::collections::{BTreeMap, HashSet};

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

const PACKED_FILE_MAGIC: [u8; 8] = *b"SBAsset6";
const INDEX_HEADER: [u8; 5] = *b"INDEX";

struct packet_asset {
    packed_file: File,
    index: BTreeMap<String, (u64, u64)>,
}

impl packet_asset {
    pub fn new(filename: &str) -> anyhow::Result<Self> {
        let mut packed_file = File::open(filename)?;

        let mut magic = [0u8; 8];
        
        packed_file.read_exact(&mut magic)?;

        if magic != PACKED_FILE_MAGIC {
            anyhow::bail!("Invalid packed file magic");
        }

        let mut index_start_bytes = [0u8; 8];

        packed_file.read_exact(&mut index_start_bytes)?;

        let index_start = u64::from_le_bytes(index_start_bytes);

        packed_file.seek(SeekFrom::Start(index_start))?;

        let mut header = [0u8; 5];
        packed_file.read_exact(&mut header)?;

        if header != INDEX_HEADER {
            anyhow::bail!("Invalid index header");
        }

        // packet_asset {}
        todo!()
    }
    
}