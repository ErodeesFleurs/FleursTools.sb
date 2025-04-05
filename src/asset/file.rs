use mlua::UserData;

#[derive(Debug, Clone)]
pub struct AssetFile {
    pub path: String,
    pub bytes: Vec<u8>,
}

impl AssetFile {
    pub fn as_string(&self) -> anyhow::Result<String> {
        Ok(self.bytes.iter().map(|&b| b as char).collect::<String>())
    }
}

impl UserData for AssetFile {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("as_string", |_, this, _: ()| {
            this.as_string().map_err(|e| mlua::Error::external(e))
        });

        methods.add_method("path", |_, this, _: ()| {
            Ok(this.path.clone())
        });
    }
}
