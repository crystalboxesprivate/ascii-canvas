use super::display;
use super::vec2;
use std::cmp;
use std::collections::HashMap;

pub struct PixelCoordinate {
  uv: (f32, f32),
}

#[derive(Default)]
pub struct ConstantBuffer {
  pub time: f64,
  pub textures: HashMap<String, display::ImageBuffer>,
}

impl ConstantBuffer {
  pub fn set_texture(&mut self, name: &str, texture: display::ImageBuffer) {
    self.textures.insert(name.to_string(), texture);
  }
  pub fn sample_texture(&self, name: &str, uv: (f32, f32)) -> u8 {
    match self.textures.get(name) {
      Some(tex) => {
        let index = (uv.0 * tex.width as f32, uv.1 * tex.height as f32);
        let index = (
          cmp::min(tex.width - 1, index.0 as usize),
          cmp::min(tex.height - 1, index.1 as usize),
        );
        tex.data[index.1 * tex.width + index.0]
      }
      None => 0,
    }
  }
}

pub trait PixelProcessor {
  fn compute(&self, coord: PixelCoordinate, data: &ConstantBuffer) -> u8;
}

pub fn compute(
  buffer: &mut display::ImageBuffer,
  data: &ConstantBuffer,
  pixel_processor: &Box<dyn PixelProcessor>,
) {
  for index in 0..buffer.data.len() {
    let index_uv = (index % buffer.width, index / buffer.width);
    let coord = PixelCoordinate {
      uv: (
        (index_uv.0 as f32) / (buffer.width as f32),
        (index_uv.1 as f32) / (buffer.height as f32),
      ),
    };
    buffer.data[index] = pixel_processor.compute(coord, &data);
  }
}

pub struct Waves {}
impl PixelProcessor for Waves {
  fn compute(&self, coord: PixelCoordinate, data: &ConstantBuffer) -> u8 {
    let uv = (coord.uv.0 * -1_f32 * 2_f32, coord.uv.1 * -1_f32 * 2_f32);
    let time = data.time as f32;
    let result = (((time + 3.0 * uv.1).cos() * 2.0 * uv.0 + time).sin()).abs();
    (result * 255_f32) as u8
  }
}

pub struct TextureSample {}
impl PixelProcessor for TextureSample {
  fn compute(&self, coord: PixelCoordinate, data: &ConstantBuffer) -> u8 {
    data.sample_texture("MainTex", coord.uv)
  }
}

pub struct WavesWithTexture {}
impl PixelProcessor for WavesWithTexture {
  fn compute(&self, coord: PixelCoordinate, data: &ConstantBuffer) -> u8 {
    let time = data.time as f32;
    let mut uv = vec2::from(coord.uv);
    let inituv = (uv - 0.5) * 0.5 + 0.5;

    uv = (uv - 0.5) * 0.2 + 0.5;
    uv = uv.lerp(
      vec2::new(
        (time * 0.1).cos().abs() * ((time * uv.y * 0.02).tan() * 0.2).abs(),
        uv.y,
      ),
      uv.y.sin() + time.sin(),
    );
    uv = inituv.lerp(uv, (time * 0.2).cos());
    data.sample_texture("MainTex", uv.as_tuple())
  }
}
