use std::f64::consts::PI;

use serde::{Deserialize, Serialize};


#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vec2D {
  pub x: f64,
  pub y: f64,
}
impl Vec2D {
  pub fn new(x:f64, y:f64) -> Self {
    Self{x,y}
  }

  pub fn at_x(x: f64) -> Self {
    Self::new(x, 0.0)
  }

  pub fn at_y(y: f64) -> Self {
    Self::new(0.0, y)
  }

  /// The origin
  pub fn zeros() -> Self {
    Self::new(0.0,0.0)
  }

  /// The coordinates on the unit circle for an angle in radians
  pub fn from_angle(angle: f64) -> Self {
    Self::new(
      f64::cos(angle),
      f64::sin(angle)
    )
  }

  /// The angle of the vector in radians
  pub fn to_angle(self) -> f64 {
    f64::atan2(self.y, self.x)
  }

  /// The angle in radians between the two points relative to the x axis.
  pub fn angle_between(self, other: Self) -> f64 {
    let diff = self - other;
    diff.to_angle()
  }

  /// The distance between two points
  pub fn dist(self, other: Self) -> f64 {
    let diff = self - other;
    diff.magnitude()
  }

  /// The length of the vector (distance from point to origin)
  pub fn magnitude(self) -> f64 {
    f64::sqrt(self.x*self.x + self.y*self.y)
  }

  /// Convert to a unit vector
  pub fn unit(self) -> Self {
    let hypo = self.magnitude();
    Self::new(self.x/hypo, self.y/hypo)
  }

  /// Scale the vector by the given magnitude
  pub fn scale(self, mult: f64) -> Self {
    Self::new(self.x*mult, self.y*mult)
  }

  /// Swap the x and y components
  pub fn swap(self) -> Self {
    Self::new(self.y, self.x)
  }

  /// Rotate the vector by the given angle in radians
  pub fn rotate(self, angle: f64) -> Self {
    let cos = f64::cos(angle);
    let sin = f64::sin(angle);
    Self {
      x: cos*self.x - sin*self.y,
      y: sin*self.x + cos*self.y
    }
  }
}
impl std::ops::Sub for Vec2D {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self::Output {
    Self::new(self.x - rhs.x, self.y - rhs.y)
  }
}
impl std::ops::Add for Vec2D {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    Self::new(self.x + rhs.x, self.y + rhs.y)
  }
}
impl std::ops::Mul for Vec2D {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self::Output {
    Self::new(self.x * rhs.x, self.y * rhs.y)
  }
}
impl std::ops::Div for Vec2D {
  type Output = Self;
  fn div(self, rhs: Self) -> Self::Output {
    Self::new(self.x / rhs.x, self.y / rhs.y)
  }
}

/// Bound an angle to the range of [-PI, PI), wrapping overflow.
pub fn bound_angle(angle: f64) -> f64 {
  bound(angle, -PI, PI)
}

/// Bound a value to the range specified by min and max, wrapping overflow.
pub fn bound(value: f64, min: f64, max: f64) -> f64 {
  ((value - min).rem_euclid(max - min)) + min
}

/// Rotate an angle by 180 degrees, keeping it within the range of [-PI, PI)
pub fn invert_angle(angle: f64) -> f64 {
  bound_angle(angle + PI)
}

/// For triangle with edges ABC, use the law of cosines to find the angle between edges A and B.
/// Arguments are the edge lengths.
pub fn find_angle(a: f64, b: f64, c: f64) -> f64 {
  let top = a*a + b*b - c*c;
  let bottom = 2.0 * a * b;
  return f64::acos(top / bottom);
}

#[cfg(test)]
mod tests {
  use std::f64::consts::PI;
  use super::{bound_angle, Vec2D};

  const F64_DIFF_MARGIN: f64 = 1.0e-15;
  const RADIANS_45: f64 = PI / 4.0;

  #[test]
  fn test_bound_angle() {
    let offsets = [0.0, 2.0*PI, 8.0*PI, -2.0*PI, -8.0*PI];
    for angle_deg in (-180..180).step_by(5) {
      let mut angle_rad = f64::to_radians(f64::from(angle_deg));

      // Test offsets that are multiples of 2PI
      for offset in offsets {
        angle_rad = angle_rad + offset;
        let bounded_rad = bound_angle(angle_rad);
        let bounded_deg = f64::to_degrees(bounded_rad);
        // Bounded(angle + offset) == angle
        assert_eq!(f64::from(angle_deg), bounded_deg.round());
      }

      // Test overflowing
      let result = bound_angle(PI + angle_rad);
      if angle_rad > 0.0 {
        // Bounded(PI + angle) == -PI + angle
        let expected = -PI + angle_rad;
        assert_eq_enough(expected, result);
      } else if angle_rad < 0.0 {
        // Bounded(PI + angle) == PI + angle
        let expected = PI + angle_rad;
        assert_eq_enough(expected, result);
      }
    }
  }

  #[test]
  fn vec2d_from_angle_to_angle() {
    // from_angle == to_angle
    for angle_deg in (-180..180).step_by(5) {
      let angle_rad = f64::to_radians(f64::from(angle_deg));
      let result = Vec2D::from_angle(angle_rad).to_angle();
      assert_eq_enough(angle_rad, result);
    }
  }

  #[test]
  fn vec2d_rotate() {
    for angle_deg in (-180..180).step_by(5) {
      let angle_rad = f64::to_radians(f64::from(angle_deg));
      let vec = Vec2D::from_angle(angle_rad); // Create unit vector at given angle
      let vec_45 = vec.rotate(RADIANS_45); // Rotate by 45 degrees
      let vec_45_unrot = vec_45.rotate(-angle_rad); // Unrotate by the given angle
      let vec_unrot = vec_45_unrot.rotate(-RADIANS_45); // Unrotate by 45 degrees
      assert_eq_enough(0.0, vec_unrot.to_angle()); // Angle should be 0
    }
  }

  #[test]
  fn vec2d_angle_between() {
    let offsets = [
      Vec2D::zeros(),
      Vec2D::from_angle(RADIANS_45),
      Vec2D::from_angle(3.0 * RADIANS_45),
      Vec2D::from_angle(-RADIANS_45),
      Vec2D::from_angle(-3.0 * RADIANS_45)
    ];
    for angle_deg in (-180..180).step_by(5) {
      let angle_rad = f64::to_radians(f64::from(angle_deg));
      for offset in offsets {
        let vec = Vec2D::from_angle(angle_rad) + offset;
        let angle_between = vec.angle_between(offset);
        assert_eq_enough(angle_rad, angle_between);
      }
    }
  }

  #[test]
  fn vec2d_unit_scale_mag() {
    let vec = Vec2D::new(1.0, -1.0).unit();
    assert_eq_enough(1.0, vec.magnitude());

    let mult = 7.0;
    let vec_scaled = vec.scale(mult);
    assert_eq_enough(mult, vec_scaled.magnitude());

    let vec_unit = vec_scaled.unit();
    assert_eq_enough(1.0, vec_unit.magnitude());
  }
  
  #[test]
  fn vec2d_dist_swap() {
    let vec = Vec2D::new(1.0, -1.0).unit();
    let dist = 7.0;

    let offset = Vec2D::new(dist, 0.0);
    let vec_offset = vec + offset;
    assert_eq_enough(dist, vec.dist(vec_offset));
    let vec_offset = vec - offset;
    assert_eq_enough(dist, vec.dist(vec_offset));

    let offset = offset.swap();
    let vec_offset = vec + offset;
    assert_eq_enough(dist, vec.dist(vec_offset));
    let vec_offset = vec - offset;
    assert_eq_enough(dist, vec.dist(vec_offset));
  }

  #[test]
  fn vec2d_add_sub_mul_div() {
    let addsub = Vec2D::new(-5.0, 7.0);
    let muldiv = Vec2D::new(-3.0, 0.5);
    let vec = Vec2D::new(1.0, -1.0);
    let vec_a = vec + addsub;
    let vec_as = vec_a - addsub;
    let vec_asm = vec_as * muldiv;
    let vec_asmd = vec_asm / muldiv;
    assert_eq_enough_vec(vec, vec_asmd);
  }

  fn assert_eq_enough_vec(a: Vec2D, b: Vec2D) {
    assert_eq_enough(a.x, b.x);
    assert_eq_enough(a.y, b.y);
  }

  fn assert_eq_enough(a: f64, b: f64) {
    let diff = a - b;
    assert!(diff.abs() < F64_DIFF_MARGIN);
  }
}



// // Shapes are any arbitrary combination of edges and vertices
// #[derive(Debug, Clone, PartialEq, Serialize)]
// pub struct Shape {
//   pub vertices: Vec<Vec2D>,
//   pub edges: Vec<(usize,usize)>,
// }
// impl Shape {
//   pub fn new() -> Self {
//     Self { vertices: Vec::new(), edges: Vec::new() }
//   }
//   pub fn add_vertex(&mut self, vertex: Vec2D) {
//     self.vertices.push(vertex);
//   }
//   pub fn add_edge(&mut self, from: usize, to: usize) {
//     if from >= self.vertices.len() {
//       println!("Warning: Edge added from index {}, which does not exist", from);
//     }
//     if to >= self.vertices.len() {
//       println!("Warning: Edge added to index {}, which does not exist", to);
//     }
//     self.edges.push((from, to));
//   }
// }

// #[tauri::command]
// pub fn test_geometry() -> Vec<Shape> {
//   let mut shapes = Vec::new();

//   let mut a: Shape = Shape::new();
//   a.add_edge(0,1);
//   a.add_vertex(Vec2D::new(42.0, 66.0));
//   a.add_vertex(Vec2D::new(69.0, 70.0));
//   a.add_vertex(Vec2D::new(42.25, 76.0));
//   a.add_edge(2,3);
//   a.add_vertex(Vec2D::new(69.25, 80.0));
//   a.add_edge(3,0);
//   a.add_edge(1,2);
//   shapes.push(a);


//   let mut c: Shape = Shape::new();
//   c.add_vertex(Vec2D::new(42.0, 86.0));
//   c.add_vertex(Vec2D::new(69.0, 90.0));
//   c.add_edge(0,1);
//   shapes.push(c);

//   return shapes;
// }