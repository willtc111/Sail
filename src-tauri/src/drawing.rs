use serde::Serialize;

use crate::{
  geometry::Vec2D, physics::Force, ship::{SquareRigShip, HULL_LENGTH, HULL_WIDTH, RUDDER_LENGTH, SAIL_WIDTH}
};

// Ship drawing constants
const HALF_HULL_WIDTH: f64 = HULL_WIDTH / 2.0;
const HALF_HULL_LENGTH: f64 = HULL_LENGTH / 2.0;
const HALF_SAIL_WIDTH: f64 = SAIL_WIDTH / 2.0;

#[derive(Debug, Clone, Serialize)]
pub struct SquareRigShipShape {
  center: Vec2D,
  hull: Vec<Vec2D>,
  sail: Vec<Vec2D>,
  rudder: Vec<Vec2D>,
}
impl SquareRigShipShape {
  pub fn default(scale: f64) -> Self {
    let center = Vec2D::new(0.0,0.0);

    let mut hull = Vec::new();
    hull.push(Vec2D::new(-HALF_HULL_LENGTH * scale, HALF_HULL_WIDTH * scale));
    hull.push(Vec2D::new(HALF_HULL_LENGTH * scale, HALF_HULL_WIDTH * scale));
    hull.push(Vec2D::new(HALF_HULL_LENGTH * scale, -HALF_HULL_WIDTH * scale));
    hull.push(Vec2D::new(-HALF_HULL_LENGTH * scale, -HALF_HULL_WIDTH * scale));

    let mut sail = Vec::new();
    sail.push(Vec2D::new(HALF_SAIL_WIDTH * scale, 0.0));
    sail.push(Vec2D::new(-HALF_SAIL_WIDTH * scale, 0.0));

    let mut rudder = Vec::new();
    rudder.push(Vec2D::new(0.0, 0.0));
    rudder.push(Vec2D::new(-RUDDER_LENGTH * scale, 0.0));

    Self {
      center,
      hull,
      sail,
      rudder,
    }
  }

  pub fn new(ship: &SquareRigShip, default_ship: &Self) -> Self {
    let hull = default_ship.hull.iter()
      .map(|p|
        p.rotate(ship.heading) + ship.loc
      ).collect();

    let sail = default_ship.sail.iter()
      .map(|p|
        p.rotate(ship.sail_angle + ship.heading) + ship.loc
      ).collect();

    let rudder_offset = Vec2D::new(-HALF_HULL_LENGTH, 0.0);
    let rudder = default_ship.rudder.iter()
      .map(|p|
        (p.rotate(ship.rudder_angle) + rudder_offset).rotate(ship.heading) + ship.loc
      ).collect();

    Self {
      center: ship.loc,
      hull,
      sail,
      rudder
    }
  }
}

#[derive(Debug, Clone, Serialize)]
pub struct Arrow {
  start: Vec2D,
  end: Vec2D,
  width: f32,
  head_size: u32,
}
impl Arrow {
  pub fn new(start: Vec2D, end: Vec2D, width: f32, head_size: u32,) -> Self {
    Self {
      start,
      end,
      width,
      head_size,
    }
  }
  pub fn from_force(force: &Force) -> Self {
    Self::new(force.loc, force.loc + force.vec, 0.5, 1)
  }
}

#[derive(Debug, Clone, Serialize)]
pub struct PhysicsShapes {
  pub ship: SquareRigShipShape,
  pub forces: Vec<Arrow>
}
