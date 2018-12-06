use std::cmp;

pub struct ImageBuffer {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

impl ImageBuffer {
    pub fn new(w: usize, h: usize) -> ImageBuffer {
        ImageBuffer {
            width: w,
            height: h,
            data: vec![255; w * h],
        }
    }
}

pub fn clear_screen() {
    print!("{}[2J", 27 as char);
}

pub fn draw(screen_buffer: &ImageBuffer, greyscale_map: &str) {
    let greyscale_map: Vec<char> = greyscale_map.chars().collect();
    for y in 0..screen_buffer.height {
        let scanline = &screen_buffer.data[screen_buffer.width * y..screen_buffer.width * (y + 1)];
        let scanline_transformed: String = scanline
            .iter()
            .map(|x| {
                let index = (*x as f32) / 255.0 * (greyscale_map.len() as f32);
                let index = cmp::min(greyscale_map.len() - 1, index as usize);
                greyscale_map[index]
            }).collect();
        println!("{}", scanline_transformed);
    }
}
