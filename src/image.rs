use image::GenericImageView;
use mlua::{MetaMethod, UserData};
use std::collections::HashMap;

pub struct Image {
    img: image::DynamicImage,
}

impl Image {
    pub fn new(path: &str) -> Self {
        Self {
            img: image::open(path).unwrap(),
        }
    }

    pub fn weight(&self) -> u32 {
        self.img.width()
    }

    pub fn height(&self) -> u32 {
        self.img.height()
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> image::Rgba<u8> {
        self.img.get_pixel(x, y)
    }
}

impl UserData for Image {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("width", |_, this, _: ()| Ok(this.weight()));
        methods.add_method("height", |_, this, _: ()| Ok(this.height()));
        methods.add_meta_function(MetaMethod::Call, |_, path: String| Ok(Image::new(&path)));
    }

    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        let _ = fields;
    }
}

pub struct ImageParseOptions {
    pub skip_transparent: bool,
}

pub fn to_color_table(img: &Image, options: ImageParseOptions) -> Vec<Vec<String>> {
    let mut rows = vec![];
    for y in 0..img.height() {
        rows.push(vec![]);
        for x in 0..img.weight() {
            let pixel = img.get_pixel(x, y);
            if options.skip_transparent && pixel[3] == 0 {
                rows[y as usize].push("".to_string());
                continue;
            }
            let hex = to_hex(&[
                pixel[0] as usize,
                pixel[1] as usize,
                pixel[2] as usize,
                pixel[3] as usize,
            ]);
            rows[y as usize].push(hex);
        }
    }
    rows
}

pub fn to_hex(arr: &[usize]) -> String {
    if arr.len() == 3 || arr[3] == 255 {
        format!("{:02x}{:02x}{:02x}", arr[0], arr[1], arr[2])
    } else {
        format!("{:02x}{:02x}{:02x}{:02x}", arr[0], arr[1], arr[2], arr[3])
    }
}

pub fn diffrent(from: Vec<Vec<String>>, to: Vec<Vec<String>>) -> HashMap<String, String> {
    let mut swaps = HashMap::new();
    for (y, row) in from.iter().enumerate() {
        for (x, a) in row.iter().enumerate() {
            let b = to[y][x].clone();
            if b.is_empty() {
                continue;
            }
            swaps.insert(a.clone(), b);
        }
    }
    swaps
}
