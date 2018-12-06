use std::time::Duration;
use std::thread;

mod display;
mod pixel_processing;

fn main() {
    let mut screen_buffer = display::ImageBuffer::new(60, 30);
    let mut time = 0.0;
    let mut pixel_processor = pixel_processing::Waves {
        data: pixel_processing::PixelProcessorData::default(),
    };

    loop {
        display::clear_screen();
        pixel_processor.data.time = time;
        pixel_processing::compute(&mut screen_buffer, &pixel_processor);
        display::draw(&screen_buffer, "█▇▆▅▄▃▂▁ ");
        thread::sleep(Duration::from_millis(1));
        time += 0.0023;
    };
}
