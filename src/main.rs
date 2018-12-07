extern crate ascii_canvas;

fn main() {
  let mut app = ascii_canvas::App::new(
    ascii_canvas::read_options_from_args(),
    Box::new(ascii_canvas::pixel_processing::WavesWithTexture {}),
  );

  match ascii_canvas::bmp_reader::load_rgb_24bit("assets/hello.bmp") {
    Ok(texture) => app.shared_data.set_texture("MainTex", texture),
    Err(error_message) => println!("{}", error_message),
  };

  app.run();
}
