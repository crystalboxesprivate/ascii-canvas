use super::display;
use std::fs::File;
use std::io::Read;

pub fn load_rgb_24bit(filename: &str) -> Result<display::ImageBuffer, &str> {
  let mut file = if let Ok(valid_file) = File::open(filename) {
    valid_file
  } else {
    return Err("File not found");
  };

  let mut header_info = [0_u8; 54];
  if let Err(_) = file.read(&mut header_info) {
    return Err("Couldn't read header");
  }

  fn get_mut_u8_ptr<T>(reference: &mut T) -> *mut u8 {
    (reference as *mut T) as *mut u8
  }

  let write_uint_from_header = |offset: usize, stride: usize, ptr: *mut u8| unsafe {
    for x in 0..stride {
      *ptr.offset(x as isize) = header_info[offset + x];
    }
  };

  let mut resolution = (0_u32, 0_u32);
  let mut bit_count = 0_u16;

  write_uint_from_header(18, 4, get_mut_u8_ptr(&mut resolution.0));
  write_uint_from_header(22, 4, get_mut_u8_ptr(&mut resolution.1));
  let resolution = (resolution.0 as usize, resolution.1 as usize);

  write_uint_from_header(28, 2, get_mut_u8_ptr(&mut bit_count));
  if bit_count != 24 {
    return Err("Not a 24 bit bmp.")
  }

  let pixel_count = resolution.0 * resolution.1;
  let size = pixel_count * 3;
  let mut pixels_raw = vec![0_u8; size];
  if let Err(_) = file.read(&mut pixels_raw) {
    return Err("Couldn't read pixels")
  }
  let pixels_raw = pixels_raw;

  let mut pixels = vec![0_u8; pixel_count];
  for y in 0..resolution.1 {
    for x in 0..resolution.0 {
      let index = y * resolution.0 + x;
      let rgb = (
        pixels_raw[index * 3 + 2] as f32 / 255.0,
        pixels_raw[index * 3 + 1] as f32 / 255.0,
        pixels_raw[index * 3 + 0] as f32 / 255.0,
      );
      let greyscale = 0.3 * rgb.0 + 0.59 * rgb.1 * 0.11 * rgb.2;
      let index = (resolution.1 - 1 - y) * resolution.0 + x;
      pixels[index] = (greyscale * 255.0) as u8;
    }
  }

  Ok(display::ImageBuffer {
    width: resolution.0 as usize,
    height: resolution.1 as usize,
    data: pixels,
  })
}
