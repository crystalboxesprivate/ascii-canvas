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

pub fn draw(screen_buffer: &ImageBuffer, greyscale_map: &str, ascii_buffer: &mut Vec<char>) {
    let greyscale_map: Vec<char> = greyscale_map.chars().collect();
    let char_buffer_width = screen_buffer.width + 1;

    for y in 0..screen_buffer.height {
        for x in 0..screen_buffer.width {
            let index = y * screen_buffer.width + x;
            let remapped_value =
                (screen_buffer.data[index] as f32) / 255.0 * (greyscale_map.len() as f32);
            let remapped_value = cmp::min(greyscale_map.len() - 1, remapped_value as usize);

            let index = y * char_buffer_width + x;
            ascii_buffer[index] = greyscale_map[remapped_value];
        }
        ascii_buffer[y * char_buffer_width + char_buffer_width - 1] = '\n';
    }

    let dst: String = ascii_buffer.iter().collect();
    print!("{}", dst);
}
