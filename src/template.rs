use crate::image::to_hex;

pub fn create(frame_width: u8, frame_height: u8, frames: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let vertical_frames = frames.len() as u8;
    let horizontal_frames = frames[0].len() as u8;

    let v = (vertical_frames as usize) * (frame_height as usize);
    // Build arrray
    let mut rows = vec![
        vec![String::new(); (horizontal_frames as usize) * (frame_width as usize)];
        v as usize
    ];

    // For every frame
    for w in 0..horizontal_frames {
        for h in 0..vertical_frames {
            // Frame identifier
            let id = &frames[(vertical_frames as usize) - 1 - (h as usize)][w as usize];
            if id.is_empty() {
                continue;
            }

            // For every pixel in frame
            for x in 0..frame_width {
                for y in 0..frame_height {
                    let pixel = to_hex(&[
                        x.try_into().unwrap(),
                        u8::from_str_radix(id, 16).unwrap(),
                        y.try_into().unwrap(),
                        0,
                    ]);
                    rows[(v as usize)
                        - 1
                        - ((h as usize) * (frame_height as usize) + (y as usize))]
                        [(w as usize) * (frame_width as usize) + (x as usize)] = pixel;
                }
            }
        }
    }

    rows
}
