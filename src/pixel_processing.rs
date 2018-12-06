use super::display;

pub struct PixelCoordinate {
    index: (i32, i32),
    uv: (f32, f32)
}

#[derive(Default)]
pub struct PixelProcessorData {
    pub time: f64,
}

pub trait PixelProcessor {
    fn compute(&self, coord: PixelCoordinate, buffer: &display::ImageBuffer) -> u8;
}

pub fn compute(buffer: &mut display::ImageBuffer, pixel_processor: &dyn PixelProcessor) {
    for index in 0..buffer.data.len() {
        let index_uv = (index % buffer.width, index / buffer.height);
        let coord = PixelCoordinate { 
            index: (index_uv.0 as i32, index_uv.1 as i32),
            uv: ((index_uv.0 as f32) / (buffer.width as f32), 
            (index_uv.1 as f32) / (buffer.height as f32)),
        };
        buffer.data[index] = pixel_processor.compute(coord, &buffer);
    }
}

// test stuff
pub struct Waves {
    pub data: PixelProcessorData,
}

impl PixelProcessor for Waves {
    fn compute(&self, coord: PixelCoordinate, buffer: &display::ImageBuffer) -> u8 {
        let uv = (coord.uv.0 * -1_f32 * 2_f32,
        coord.uv.1 * -1_f32 * 2_f32);
        let time = self.data.time as f32;
        let result = (((time + 3.0 * uv.1).cos() * 2.0 * uv.0 + time).sin()).abs();
        (result * 255_f32) as u8
    }
}
