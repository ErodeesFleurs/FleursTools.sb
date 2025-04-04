use mlua::UserData;

pub struct AssetFile {
    pub path: String,
    pub bytes: Vec<u8>,
}

impl UserData for AssetFile {
    
}