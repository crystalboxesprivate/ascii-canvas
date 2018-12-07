use std::ops;

#[derive(Copy)]
pub struct Vec2 {
  pub x: f32,
  pub y: f32,
}
impl Clone for Vec2 {
  fn clone(&self) -> Vec2 {
    *self
  }
}

pub fn from(from: (f32, f32)) -> Vec2 {
  Vec2 {
    x: from.0,
    y: from.1,
  }
}
pub fn new(x: f32, y: f32) -> Vec2 {
  Vec2 { x: x, y: y }
}

impl Vec2 {
  pub fn lerp(&self, b: Vec2, t: f32) -> Vec2 {
    (b - *self) * t + *self
  }

  pub fn as_tuple(&self) -> (f32, f32) {
    (self.x, self.y)
  }
}

impl ops::MulAssign<f32> for Vec2 {
  fn mul_assign(&mut self, rhs: f32) {
    self.x *= rhs;
    self.y *= rhs;
  }
}

impl ops::Mul<f32> for Vec2 {
  type Output = Vec2;
  fn mul(self, rhs: f32) -> Vec2 {
    Vec2 {
      x: self.x * rhs,
      y: self.y * rhs,
    }
  }
}

impl ops::Sub<f32> for Vec2 {
  type Output = Vec2;
  fn sub(self, rhs: f32) -> Vec2 {
    Vec2 {
      x: self.x - rhs,
      y: self.y - rhs,
    }
  }
}

impl ops::Sub<Vec2> for Vec2 {
  type Output = Vec2;
  fn sub(self, rhs: Vec2) -> Vec2 {
    Vec2 {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    }
  }
}

impl ops::Mul<Vec2> for Vec2 {
  type Output = Vec2;
  fn mul(self, rhs: Vec2) -> Vec2 {
    Vec2 {
      x: self.x * rhs.x,
      y: self.y * rhs.y,
    }
  }
}

impl ops::Add<Vec2> for f32 {
  type Output = Vec2;
  fn add(self, rhs: Vec2) -> Vec2 {
    Vec2 {
      x: self + rhs.x,
      y: self + rhs.y,
    }
  }
}

impl ops::Add<f32> for Vec2 {
  type Output = Vec2;
  fn add(self, rhs: f32) -> Vec2 {
    Vec2 {
      x: self.x + rhs,
      y: self.y + rhs,
    }
  }
}

impl ops::Add<Vec2> for Vec2 {
  type Output = Vec2;
  fn add(self, rhs: Vec2) -> Vec2 {
    Vec2 {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}
