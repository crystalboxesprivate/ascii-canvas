use std::env;
use std::thread;
use std::time::Duration;

pub mod display;
pub mod pixel_processing;
pub mod bmp_reader;
pub mod vec2;

pub struct AppOptions {
  width: usize,
  height: usize,
  frame_delay_ms: f64,
  ascii_gradient_type: u32,
}

impl AppOptions {
  fn new() -> AppOptions {
    AppOptions {
      width: 80,
      height: 25,
      frame_delay_ms: 33.333333,
      ascii_gradient_type: 3,
    }
  }
}

pub struct App {
  screen_buffer: display::ImageBuffer,
  ascii_buffer: Vec<char>,
  pixel_processor: Box<dyn pixel_processing::PixelProcessor>,
  pub shared_data: pixel_processing::ConstantBuffer,
  refresh_rate: f64, // ms
  gradient_map: String,
}

impl App {
  pub fn new(
    app_options: AppOptions,
    pixel_processor: Box<dyn pixel_processing::PixelProcessor>,
  ) -> App {
    let gradient_map = match app_options.ascii_gradient_type {
      0 => "@#*+=:-. ",
      1 => "@MBHENR#KWXDFPQASUZbdehx*8Gm&04LOVYkpq5Tagns69owz$CIu23Jcfry%1v7l+it[] {}?j|()=~!-/<>\"^_';,:`. ",
      2 => "ÆÑÊŒØMÉËÈÃÂWQBÅæ#NÁþEÄÀHKRŽœXgÐêqÛŠÕÔA€ßpmãâG¶øðé8ÚÜ$ëdÙýèÓÞÖåÿÒb¥FDñáZPäšÇàhû§ÝkŸ®S9žUTe6µOyxÎ¾f4õ5ôú&aü™2ùçw©Y£0VÍL±3ÏÌóC@nöòs¢u‰½¼‡zJƒ%¤Itocîrjv1lí=ïì<>i7†[¿?×}*{+()\\/»«•¬|!¡÷¦¯—^ª„”“~³º²–°­¹‹›;:’‘‚’˜ˆ¸…·¨´` ",
      3 => "█▓▒░ ",
      _ => "█▇▆▅▄▃▂▁ ",
    };

    App {
      screen_buffer: display::ImageBuffer::new(app_options.width, app_options.height),
      ascii_buffer: vec![' '; (app_options.width + 1) * app_options.height],
      pixel_processor: pixel_processor,
      shared_data: pixel_processing::ConstantBuffer::default(),
      refresh_rate: app_options.frame_delay_ms,
      gradient_map: String::from(gradient_map),
    }
  }
  pub fn run(&mut self) {
    loop {
      display::clear_screen();
      pixel_processing::compute(
        &mut self.screen_buffer,
        &self.shared_data,
        &self.pixel_processor,
      );
      display::draw(
        &self.screen_buffer,
        &self.gradient_map,
        &mut self.ascii_buffer,
      );
      self.wait_and_update_time();
    }
  }

  fn wait_and_update_time(&mut self) {
    self.shared_data.time += self.refresh_rate * 0.001_f64;
    thread::sleep(Duration::from_millis(self.refresh_rate as u64));
  }
}

pub fn read_options_from_args() -> AppOptions {
  let args: Vec<String> = env::args().collect();
  let args = &args[1..args.len()];

  let mut app_options = AppOptions::new();

  fn warning_msg() {
    println!("Couldn't parse command line arguments.")
  }

  fn get_number<T: std::str::FromStr>(opt: Option<&String>, default: T) -> T {
    if let None = opt {
      return default;
    };
    if let Ok(parsed) = opt.unwrap().parse::<T>() {
      return parsed;
    }
    warning_msg();
    default
  };

  let mut arg = args.into_iter();

  loop {
    let next_iter = arg.next();

    if let None = next_iter {
      break;
    }

    match next_iter.unwrap().as_ref() {
      "s" => {
        app_options.width = get_number(arg.next(), app_options.width);
        app_options.height = get_number(arg.next(), app_options.height);
      }
      "r" => app_options.frame_delay_ms = get_number(arg.next(), app_options.frame_delay_ms),
      "t" => {
        app_options.ascii_gradient_type = get_number(arg.next(), app_options.ascii_gradient_type)
      }
      _ => {
        warning_msg();
        break;
      }
    }
  }

  app_options
}

#[test]
fn bmp_read() {
    match bmp_reader::load_rgb_24bit("assets/grid_rectangle.bmp") {
      Ok(x) => assert_eq!(x.width, 400),
      Err(x) => panic!(x),
    };
}
