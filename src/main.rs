extern crate ascii_canvas;

// TODO
// Arguments for options
// BMP reader and processor
// OBJ loader

fn main() {
  let rows = 80;
  let cols = 35;
  let frame_delay_ms = 22.0;

  let mut app = ascii_canvas::App::new(
    rows,
    cols,
    frame_delay_ms,
    Box::new(ascii_canvas::pixel_processing::Waves {}),
  );
  app.run();
}
