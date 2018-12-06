use std::rc::Rc;
use std::thread;
use std::time::Duration;

pub mod display;
pub mod pixel_processing;

pub struct App {
  screen_buffer: display::ImageBuffer,
  ascii_buffer: Vec<char>,
  pixel_processor: Box<dyn pixel_processing::PixelProcessor>,
  pixel_processor_data: pixel_processing::ConstantBuffer,
  screen_refresh_rate: f64, // ms
}

impl App {
  pub fn new(
    width: usize,
    height: usize,
    screen_refresh_rate: f64,
    pixel_processor: Box<dyn pixel_processing::PixelProcessor>,
  ) -> App {
    App {
      screen_buffer: display::ImageBuffer::new(width, height),
      ascii_buffer: vec![' '; (width + 1) * height],
      pixel_processor: pixel_processor,
      pixel_processor_data: pixel_processing::ConstantBuffer::default(),
      screen_refresh_rate: screen_refresh_rate,
    }
  }
  pub fn run(&mut self) {
    loop {
      display::clear_screen();
      pixel_processing::compute(&mut self.screen_buffer, &self.pixel_processor_data, &self.pixel_processor);
      display::draw(
        &self.screen_buffer,
        "█▇▆▅▄▃▂▁ ",
        &mut self.ascii_buffer,
      );
      self.wait_and_update_time();
    }
  }

  fn wait_and_update_time(&mut self) {
    self.pixel_processor_data.time += self.screen_refresh_rate * 0.001_f64;
    thread::sleep(Duration::from_millis(self.screen_refresh_rate as u64));
  }
}
