use std::collections::HashMap;

use mlua::AnyUserData;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Image {
    data: *mut u8,
    width: u32,
    height: u32,
    format: PixelFormat,
}

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum PixelFormat {
    RGB24 = 0,
    RGBA32 = 1,
    BGR24 = 2,
    BGRA32 = 3,
    RGBF = 4,
    RGBAF = 5,
}

#[allow(dead_code)]
impl Image {
    pub fn new(data: *mut u8, width: u32, height: u32, format: PixelFormat) -> Self {
        Self {
            data,
            width,
            height,
            format,
        }
    }

    pub fn weight(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn bytes_per_pixel(&self) -> usize {
        match self.format {
            PixelFormat::RGB24 | PixelFormat::BGR24 => 3,
            PixelFormat::RGBA32 | PixelFormat::BGRA32 => 4,
            PixelFormat::RGBF => 12,
            PixelFormat::RGBAF => 16,
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> [u8; 4] {
        let offset = ((y * self.width + x) * self.bytes_per_pixel() as u32) as usize;
        let mut pixel: [u8; 4] = [0; 4];
        match self.bytes_per_pixel() {
            4 => unsafe {
                pixel[0] = *self.data.add(offset);
                pixel[1] = *self.data.add(offset + 1);
                pixel[2] = *self.data.add(offset + 2);
                pixel[3] = *self.data.add(offset + 3);
            },

            3 => {
                pixel[0] = unsafe { *self.data.add(offset) };
                pixel[1] = unsafe { *self.data.add(offset + 1) };
                pixel[2] = unsafe { *self.data.add(offset + 2) };
                pixel[3] = 255;
            }
            _ => panic!("Unsupported bytes per pixel"),
        }
        pixel
    }
}

pub fn read_image(userdata: AnyUserData) -> &'static Image {
    let img: &Image;
    unsafe {
        let raw_ptr = userdata.to_pointer() as *const Image;
        img = &*raw_ptr;
    }
    img
}

pub struct ImageParseOptions {
    pub skip_transparent: bool,
}

pub fn to_color_table(img: &Image, options: ImageParseOptions) -> Vec<Vec<String>> {
    let mut rows = vec![vec!["".to_string(); img.weight() as usize]; img.height() as usize];
    for y in 0..img.height() {
        for x in 0..img.weight() {
            let pixel = img.get_pixel(x, y);
            if options.skip_transparent && pixel[3] == 0 {
                continue;
            }
            let hex = to_hex(&[pixel[0], pixel[1], pixel[2], pixel[3]]);
            rows[(img.height() - 1 - y) as usize][x as usize] = hex;
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
