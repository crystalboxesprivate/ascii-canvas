extern crate ascii_canvas;
// TODO
// BMP reader

fn main() {
  let mut app = ascii_canvas::App::new(
    ascii_canvas::read_options_from_args(),
    Box::new(ascii_canvas::pixel_processing::Waves {}),
  );
  app.run();
}
