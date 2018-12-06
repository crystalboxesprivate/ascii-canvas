pub struct ScreenBuffer {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl ScreenBuffer {
    pub fn new(w: usize, h: usize) -> ScreenBuffer {
        ScreenBuffer {
            width: w,
            height: h,
            data: vec![255; w * h],
        }
    }
}

pub fn clear_screen() {
    print!("{}[2J", 27 as char);
}

pub fn draw(screen_buffer: &ScreenBuffer, greyscale_map: &str) {
    let greyscale_map: Vec<char> = greyscale_map.chars().collect();
    for y in 0..screen_buffer.height {
        let scanline = &screen_buffer.data[screen_buffer.width * y..screen_buffer.width * (y + 1)];
        let scanline_transformed: String = scanline
            .iter()
            .map(|x| {
                let index = (*x as f32) / 255.0 * (greyscale_map.len() as f32);
                greyscale_map[index as usize]
            }).collect();
        println!("{}", scanline_transformed);
    }
}
