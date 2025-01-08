use crate::image::to_hex;

pub fn create(
    frame_width: usize,
    frame_height: usize,
    frames: Vec<Vec<String>>,
) -> Vec<Vec<String>> {
    let vertical_frames = frames.len();
    let horizontal_frames = frames[0].len();

    let v = vertical_frames * frame_height;
    // Build arrray
    let mut rows = vec![vec![String::new(); horizontal_frames * frame_width]; v as usize];

    // For every frame
    for w in 0..horizontal_frames {
        for h in 0..vertical_frames {
            // Frame identifier
            let id = &frames[vertical_frames - 1 - h][w];
            if id.is_empty() {
                continue;
            }

            // For every pixel in frame
            for x in 0..frame_width {
                for y in 0..frame_height {
                    let pixel = to_hex(&[x, u32::from_str_radix(id, 16).unwrap() as usize, y, 0]);
                    rows[v as usize - 1 - (h * frame_height + y) as usize]
                        [w as usize * frame_width as usize + x as usize] = pixel;
                }
            }
        }
    }

    rows
}
