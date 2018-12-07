extern crate ascii_canvas;

fn main() {
  let mut app = ascii_canvas::App::new(
    ascii_canvas::read_options_from_args(),
    Box::new(ascii_canvas::pixel_processing::TextureSample {}),
  );

  match ascii_canvas::bmp_reader::load_rgb_24bit("assets/grid_rectangle.bmp") {
    Ok(x) => app.shared_data.set_texture("MainTex", x),
    Err(x) => println!("{}", x),
  };

  app.run();
}
