use serde::Serialize;

use crate::{
  geometry::Vec2D, physics::Force, ship::AdjustableShip
};

// Ship drawing constants
const LINE_THICKNESS: f64 = 0.5;

#[derive(Debug, Clone, Serialize)]
pub struct AdjustableShipShape {
  center: Vec2D,
  hull: Polygon,
  sails: Vec<Polygon>,
  rudder: Polygon,
}
impl AdjustableShipShape {
  pub fn new(ship: &AdjustableShip) -> Self {
    let hull = Polygon::centered_rectangle(
      ship.specs.hull_length, ship.specs.hull_width,
      ship.heading, ship.loc
    );

    let mut sails: Vec<Polygon> = Vec::new();
    for sail_index in 0..ship.specs.sails.len() {
      let sail = &ship.specs.sails[sail_index];
      sails.push(Polygon::line(
        sail.width, LINE_THICKNESS,
        ship.sail_angles[sail_index], Vec2D::at_x(sail.mast_offset),
        ship.heading, ship.loc
      ));
    }

    let rudder = Polygon::line(
      ship.specs.rudder_length, LINE_THICKNESS,
      ship.rudder_angle, Vec2D::at_x(-ship.specs.hull_length*0.5),
      ship.heading, ship.loc
    );

    Self {
      center: ship.loc,
      hull,
      sails,
      rudder
    }
  }
}


impl Vec2D {
  pub fn transform(self, angle: f64, location: Self) -> Self {
    return self.rotate(angle) + location;
  }
}

#[derive(Debug, Clone, Serialize)]
pub struct Polygon {
  points: Vec<Vec2D>
}
impl Polygon {
  /// Create a rectangle with 0,0 at the center, moved to the transformed location
  pub fn centered_rectangle(length: f64, width: f64, heading: f64, location: Vec2D) -> Self {
    let half_length = length * 0.5;
    let half_width = width * 0.5;
    let mut points = Vec::new();
    points.push(Vec2D::new(-half_length, half_width).transform(heading, location));
    points.push(Vec2D::new(half_length, half_width).transform(heading, location));
    points.push(Vec2D::new(half_length, -half_width).transform(heading, location));
    points.push(Vec2D::new(-half_length, -half_width).transform(heading, location));
    Self { points }
  }
  /// Create a line from (0,0) to (-length,0) with given thickness, moved to the transformed location
  pub fn line(length: f64, thickness: f64, angle: f64, offset: Vec2D, heading: f64, location: Vec2D) -> Self {
    let half_thickness = thickness * 0.5;
    let mut points = Vec::new();
    points.push(Vec2D::new(0.0, half_thickness).transform(angle, offset).transform(heading, location));
    points.push(Vec2D::new(-length, half_thickness).transform(angle, offset).transform(heading, location));
    points.push(Vec2D::new(-length, -half_thickness).transform(angle, offset).transform(heading, location));
    points.push(Vec2D::new(0.0, -half_thickness).transform(angle, offset).transform(heading, location));
    Self { points }
  }
}

#[derive(Debug, Clone, Serialize)]
pub struct Arrow {
  name: String,
  start: Vec2D,
  end: Vec2D,
  width: f64,
  head_size: f64,
}
impl Arrow {
  pub fn new(name: String, start: Vec2D, end: Vec2D, width: f64, head_size: f64) -> Self {
    Self {
      name,
      start,
      end,
      width,
      head_size,
    }
  }
  pub fn from_force(force: &Force) -> Self {
    Self::new(force.name.clone(), force.loc, force.loc + force.vec, LINE_THICKNESS, 1.0)
  }
}

#[derive(Debug, Clone, Serialize)]
pub struct PhysicsShapes {
  pub ship: AdjustableShipShape,
  pub forces: Vec<Arrow>
}
