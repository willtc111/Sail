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

  /// The angle between the two points relative to the x axis.
  /// Result is in radians
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

// Shapes are any arbitrary combination of edges and vertices
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Shape {
  pub vertices: Vec<Vec2D>,
  pub edges: Vec<(usize,usize)>,
}
impl Shape {
  pub fn new() -> Self {
    Self { vertices: Vec::new(), edges: Vec::new() }
  }
  pub fn add_vertex(&mut self, vertex: Vec2D) {
    self.vertices.push(vertex);
  }
  pub fn add_edge(&mut self, from: usize, to: usize) {
    if from >= self.vertices.len() {
      println!("Warning: Edge added from index {}, which does not exist", from);
    }
    if to >= self.vertices.len() {
      println!("Warning: Edge added to index {}, which does not exist", to);
    }
    self.edges.push((from, to));
  }
}

#[tauri::command]
pub fn test_geometry() -> Vec<Shape> {
  let mut shapes = Vec::new();

  let mut a: Shape = Shape::new();
  a.add_edge(0,1);
  a.add_vertex(Vec2D::new(42.0, 66.0));
  a.add_vertex(Vec2D::new(69.0, 70.0));
  a.add_vertex(Vec2D::new(42.25, 76.0));
  a.add_edge(2,3);
  a.add_vertex(Vec2D::new(69.25, 80.0));
  a.add_edge(3,0);
  a.add_edge(1,2);
  shapes.push(a);


  let mut c: Shape = Shape::new();
  c.add_vertex(Vec2D::new(42.0, 86.0));
  c.add_vertex(Vec2D::new(69.0, 90.0));
  c.add_edge(0,1);
  shapes.push(c);

  return shapes;
}