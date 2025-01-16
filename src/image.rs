use image::GenericImageView;
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
            let hex = to_hex(&[pixel[0], pixel[1], pixel[2], pixel[3]]);
            rows[y as usize].push(hex);
        }
    }
    rows
}

pub fn to_hex(arr: &[u8]) -> String {
    let mut c_arr = arr;
    if arr.len() == 4 && arr[3] == 255 {
        c_arr = &arr[0..3];
    }
    let shorten = c_arr.iter().all(|&x| x % 17 == 0);
    let s = c_arr
        .iter()
        .map(|x| {
            let mut s = format!("{:02x}", x);
            if shorten {
                s = s.chars().last().unwrap().to_string();
            }
            s
        })
        .collect::<String>();
    s
}

pub fn diffrent(from: Vec<Vec<String>>, to: Vec<Vec<String>>) -> HashMap<String, String> {
    let mut swaps = HashMap::new();
    for (y, row) in from.iter().enumerate() {
        for (x, a) in row.iter().enumerate() {
            let b = to[y][x].clone();
            if b.is_empty() || a.is_empty() {
                continue;
            }
            swaps.insert(a.clone(), b);
        }
    }
    swaps
}
