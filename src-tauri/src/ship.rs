use std::f64::consts::PI;

use serde::Serialize;

use crate::{
  drawing::{Arrow, ForeAftRigShipShape, PhysicsShapes, SquareRigShipShape},
  geometry::{bound_angle, invert_angle, Vec2D},
  physics::{calculate_apparent_wind, calculate_drag, calculate_force, calculate_lift, Force},
  simulation::DELTA_TIME
};


pub const HULL_WIDTH: f64 = 3.0;
pub const HULL_LENGTH: f64 = 10.0;
pub const HULL_DEPTH: f64 = 0.33;
// pub const HULL_WETTED_AREA: f64 = HULL_WIDTH * HULL_LENGTH + 2.0 * HULL_DEPTH * (HULL_LENGTH + HULL_WIDTH);
pub const SAIL_WIDTH: f64 = 7.0;
pub const SAIL_WIDTH_SQUARED: f64 = SAIL_WIDTH * SAIL_WIDTH;
pub const SAIL_HEIGHT: f64 = 10.0;
pub const SAIL_AREA: f64 = SAIL_WIDTH * SAIL_HEIGHT / 2.0; // half because triangle
pub const KEEL_LENGTH: f64 = 2.0;
pub const KEEL_HEIGHT: f64 = 2.0;
pub const KEEL_AREA: f64 = KEEL_LENGTH * KEEL_HEIGHT;
pub const RUDDER_LENGTH: f64 = 1.0;
pub const RUDDER_HEIGHT: f64 = 1.0;
pub const RUDDER_AREA: f64 = RUDDER_LENGTH * RUDDER_HEIGHT;
pub const MAST_OFFSET: f64 = 4.0;

pub const HULL_DENSITY: f64 = 200.0; // kg / m^3, solid wood is ~600
pub const HULL_MASS: f64 = HULL_WIDTH * HULL_LENGTH * HULL_DENSITY;
pub const INVERSE_HULL_MASS: f64 = 1.0 / HULL_MASS;

pub const DENSITY_AIR: f64 = 1.225; // kg / m^3
pub const DENSITY_WATER: f64 = 1027.0; // kg / m^3

// found empirically? when water density is included in hull drag as the value that
//  provides a maximum speed of about 1.5 * wind speed
pub const FRICTION_COEFFICIENT: f64 = 0.007;

pub trait Ship {
  fn sail(&mut self, wind_angle: f64, wind_speed: f64);
  fn forces(&mut self, wind_angle: f64, wind_speed: f64) -> Vec<Force>;
}



#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct SquareRigShip {
  pub loc: Vec2D,
  pub vel: Vec2D,
  pub rot_vel: f64,
  pub heading: f64,
  pub sail_angle: f64,
  pub rudder_angle: f64,
}
impl SquareRigShip {
  pub fn new(loc: Vec2D, vel: Vec2D, rot_vel: f64, heading: f64, sail_angle: f64, rudder_angle: f64) -> Self {
    Self { loc, vel, rot_vel, heading, sail_angle, rudder_angle }
  }
}
impl Ship for SquareRigShip {
  fn sail(&mut self, wind_angle: f64, wind_speed: f64) {
    let f = self.forces(wind_angle, wind_speed);
    f.iter().for_each(|force|
      if force.loc == self.loc {
        // Force is on the center of the ship, no rotation is needed
        self.vel = self.vel + force.vec.scale(INVERSE_HULL_MASS);
      } else {
        let offset = force.loc - self.loc;
        // Eliminate offset rotation
        let loc_rot = offset.rotate(-offset.to_angle());
        let vec_rot = force.vec.rotate(-offset.to_angle());
        let torque = vec_rot.y * loc_rot.magnitude();
        let direct_force_rot = Vec2D::new(vec_rot.x, 0.0);
        let direct_force = direct_force_rot.rotate(offset.to_angle());
        self.vel = self.vel + direct_force.scale(INVERSE_HULL_MASS);
        self.rot_vel = self.rot_vel + torque * INVERSE_HULL_MASS;
      }
    );

    // Apply velocity
    // println!("vel: {:?} {:?}", self.vel.to_angle(), self.vel.magnitude());
    self.loc = self.loc + self.vel.scale(DELTA_TIME);
    // Apply rotational velocity
    // println!("rot_vel: {:?}", self.rot_vel);
    self.heading = self.heading + self.rot_vel * DELTA_TIME;

    // println!("loc: {:?}", self.loc);
  }

  fn forces(&mut self, wind_angle: f64, wind_speed: f64) -> Vec<Force> {
    let mut forces = Vec::new();

    // Calculate angle of attack on sail
    let apparent_wind = calculate_apparent_wind(self.vel, wind_angle, wind_speed).scale(DELTA_TIME);
    if apparent_wind.magnitude() != 0.0 {
      let mut aoa = bound_angle(self.heading + self.sail_angle - apparent_wind.to_angle());
      if aoa < 0.0 {
        aoa = PI + aoa;
      }
      // Calculate sail force vectors
      let lift_magnitude = calculate_lift(aoa, SAIL_AREA, DENSITY_AIR, apparent_wind.magnitude());
      let drag_magnitude = calculate_drag(aoa, SAIL_AREA, DENSITY_AIR, apparent_wind.magnitude());
      let lift = apparent_wind.unit().rotate(PI/-2.0).scale(lift_magnitude);
      let drag = apparent_wind.unit().scale(drag_magnitude);
      forces.push(Force::new(String::from("Sail Lift"), self.loc, lift));
      forces.push(Force::new(String::from("Sail Drag"), self.loc, drag));
    }

    let water_vel = self.vel.scale(-1.0 * DELTA_TIME);
    if self.vel.magnitude() != 0.0 {
      // Calculate angle of attack on the keel/hull
      let mut aoa: f64 = self.heading - water_vel.to_angle();
      if aoa < 0.0 {
        aoa = PI + aoa;
      }
      // Calculate keel force vectors
      let lift_magnitude = calculate_lift(aoa, KEEL_AREA, DENSITY_WATER, water_vel.magnitude());
      let drag_magnitude = calculate_drag(aoa, KEEL_AREA, DENSITY_WATER, water_vel.magnitude());
      let lift = water_vel.unit().rotate(PI/-2.0).scale(lift_magnitude);
      let drag = water_vel.unit().scale(drag_magnitude);
      forces.push(Force::new(String::from("Keel Lift"), self.loc, lift));
      forces.push(Force::new(String::from("Keel Drag"), self.loc, drag));
    }
    if self.vel.magnitude() != 0.0 || self.rot_vel != 0.0 {
      // Calculate angle of attack on the rudder
      let rudder_water_rot_vel = Vec2D::new(0.0, self.rot_vel * DELTA_TIME * HULL_LENGTH * 0.5).rotate(self.heading);
      let rudder_water_velocity = water_vel + rudder_water_rot_vel;
      let mut aoa: f64 = self.heading + self.rudder_angle - rudder_water_velocity.to_angle() ;
      if aoa < 0.0 {
        aoa = PI + aoa;
      }
      // Calculate rudder force vectors
      let lift_magnitude = calculate_lift(aoa, RUDDER_AREA, DENSITY_WATER, rudder_water_velocity.magnitude());
      let drag_magnitude = calculate_drag(aoa, RUDDER_AREA, DENSITY_WATER, rudder_water_velocity.magnitude());
      let lift = rudder_water_velocity.unit().rotate(PI/-2.0).scale(lift_magnitude);
      let drag = rudder_water_velocity.unit().scale(drag_magnitude);
      let rudder_offset = Vec2D::new(-HULL_LENGTH * 0.5, 0.0);
      let rudder_loc = rudder_offset.rotate(self.heading) + self.loc;
      forces.push(Force::new(String::from("Rudder Lift"), rudder_loc, lift));
      forces.push(Force::new(String::from("Rudder Drag"), rudder_loc, drag));

      // Calculate angle of attack on the front half of the hull
      let bow_water_rot_vel = Vec2D::new(0.0, -self.rot_vel * DELTA_TIME * HULL_LENGTH * 0.25).rotate(self.heading);
      let bow_water_vel = water_vel + bow_water_rot_vel;
      let mut aoa: f64 = self.heading - bow_water_vel.to_angle();
      if aoa < 0.0 {
        aoa = PI + aoa;
      }
      let apparent_width = f64::cos(aoa).abs() * HULL_WIDTH + f64::sin(aoa).abs() * HULL_LENGTH * 0.5;
      let wetted_area = HULL_DEPTH * apparent_width;
      let drag_magnitude = calculate_force(FRICTION_COEFFICIENT, wetted_area, DENSITY_WATER, bow_water_vel.magnitude());
      let drag = bow_water_vel.unit().scale(drag_magnitude);
      let offset = Vec2D::from_angle(self.heading).scale(HULL_LENGTH * 0.5);
      forces.push(Force::new(String::from("Hull Front Drag"), self.loc + offset, drag));

      // Calculate angle of attack on the rear half of the hull
      let stern_water_rot_vel = Vec2D::new(0.0, self.rot_vel * DELTA_TIME * HULL_LENGTH * 0.25).rotate(self.heading);
      let stern_water_vel = water_vel + stern_water_rot_vel;
      let mut aoa: f64 = self.heading - stern_water_vel.to_angle();
      if aoa < 0.0 {
        aoa = PI + aoa;
      }
      let apparent_width = f64::cos(aoa).abs() * HULL_WIDTH + f64::sin(aoa).abs() * HULL_LENGTH * 0.5;
      let wetted_area = HULL_DEPTH * apparent_width;
      let drag_magnitude = calculate_force(FRICTION_COEFFICIENT, wetted_area, DENSITY_WATER, stern_water_vel.magnitude());
      let drag = stern_water_vel.unit().scale(drag_magnitude);
      let offset = Vec2D::from_angle(self.heading).scale(HULL_LENGTH * 0.5);
      forces.push(Force::new(String::from("Hull Rear Drag"), self.loc - offset, drag));
    }
    return forces;
  }
}



#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct ForeAftRigShip {
  pub loc: Vec2D,
  pub vel: Vec2D,
  pub rot_vel: f64,
  pub heading: f64,
  pub mainsheet_length: f64,
  pub sail_angle: f64,
  pub rudder_angle: f64,
}
impl ForeAftRigShip {
  pub fn new(loc: Vec2D, vel: Vec2D, rot_vel: f64, heading: f64, mainsheet_length: f64, rudder_angle: f64) -> Self {
    Self { loc, vel, rot_vel, heading, mainsheet_length, sail_angle: 0.0, rudder_angle }
  }
  fn set_sail_angle(&mut self, apparent_wind_angle: f64) {
    // Calculate the angle of the sail based on the apparent wind relative to the ship
    // law of cosines (https://www.mathsisfun.com/algebra/trig-solving-sss-triangles.html)
    // distance from the mast to the mainsheet anchor is the same as the sail size
    // angle = acos((sail_size^2 + anchor_dist^2 - mainsheet_len^2) / (2 * sail_size * anchor_dist))
    let top = SAIL_WIDTH_SQUARED + SAIL_WIDTH_SQUARED - self.mainsheet_length * self.mainsheet_length;
    let bottom = 2.0 * SAIL_WIDTH_SQUARED;
    let max_sail_angle = f64::acos(top / bottom);

    let hull_relative_apparent_wind_angle = bound_angle(invert_angle(apparent_wind_angle) - self.heading);
    self.sail_angle = if hull_relative_apparent_wind_angle.abs() <= max_sail_angle {
      // Sail is luffing
      hull_relative_apparent_wind_angle
    } else {
      // Sail is taut, and stays on the side of the wind that it is already on
      let sail_relative_apparent_wind_angle = bound_angle(hull_relative_apparent_wind_angle - self.sail_angle);
      max_sail_angle * sail_relative_apparent_wind_angle.signum()
    };
  }
}
impl Ship for ForeAftRigShip {
  fn sail(&mut self, wind_angle: f64, wind_speed: f64) {
    let f = self.forces(wind_angle, wind_speed);
    f.iter().for_each(|force| {
      // Force always changes the velocity
      self.vel = self.vel + force.vec.scale(INVERSE_HULL_MASS);
      if force.loc != self.loc {
        let offset = force.loc - self.loc;
        // Eliminate offset rotation
        let loc_rot = offset.rotate(-offset.to_angle());
        let vec_rot = force.vec.rotate(-offset.to_angle());
        let torque = vec_rot.y * loc_rot.magnitude();
        let direct_force_rot = Vec2D::new(vec_rot.x, 0.0);
        let direct_force = direct_force_rot.rotate(offset.to_angle());
        self.vel = self.vel + direct_force.scale(INVERSE_HULL_MASS);
        self.rot_vel = self.rot_vel + torque * INVERSE_HULL_MASS;
      }
    });

    // Apply velocity
    // println!("vel: {:?} {:?}", self.vel.to_angle(), self.vel.magnitude());
    self.loc = self.loc + self.vel.scale(DELTA_TIME);
    // Apply rotational velocity
    // println!("rot_vel: {:?}", self.rot_vel);
    self.heading = self.heading + self.rot_vel * DELTA_TIME;

    // println!("loc: {:?}", self.loc);
  }

  fn forces(&mut self, wind_angle: f64, wind_speed: f64) -> Vec<Force> {
    let mut forces = Vec::new();

    // Calculate angle of attack on sail
    let apparent_wind = calculate_apparent_wind(self.vel, wind_angle, wind_speed).scale(DELTA_TIME);
    if apparent_wind.magnitude() != 0.0 {
      let apparent_wind_angle = apparent_wind.to_angle();

      self.set_sail_angle(apparent_wind_angle);

      let mut aoa = bound_angle(self.heading + self.sail_angle - apparent_wind_angle);
      if aoa < 0.0 {
        aoa = PI + aoa;
      }
      // Calculate sail force vectors
      let lift_magnitude = calculate_lift(aoa, SAIL_AREA, DENSITY_AIR, apparent_wind.magnitude());
      let drag_magnitude = calculate_drag(aoa, SAIL_AREA, DENSITY_AIR, apparent_wind.magnitude());
      let lift = apparent_wind.unit().rotate(PI/-2.0).scale(lift_magnitude);
      let drag = apparent_wind.unit().scale(drag_magnitude);
      let sail_center = self.loc + Vec2D::new(MAST_OFFSET, 0.0).rotate(self.heading) + Vec2D::new(-SAIL_WIDTH*0.25, 0.0).rotate(self.heading + self.sail_angle);
      forces.push(Force::new(String::from("Sail Lift"), sail_center, lift));
      forces.push(Force::new(String::from("Sail Drag"), sail_center, drag));
    }

    let water_vel = self.vel.scale(-1.0 * DELTA_TIME);
    if self.vel.magnitude() != 0.0 {
      // Calculate angle of attack on the keel/hull
      let mut aoa: f64 = self.heading - water_vel.to_angle();
      if aoa < 0.0 {
        aoa = PI + aoa;
      }
      // Calculate keel force vectors
      let lift_magnitude = calculate_lift(aoa, KEEL_AREA, DENSITY_WATER, water_vel.magnitude());
      let drag_magnitude = calculate_drag(aoa, KEEL_AREA, DENSITY_WATER, water_vel.magnitude());
      let lift = water_vel.unit().rotate(PI/-2.0).scale(lift_magnitude);
      let drag = water_vel.unit().scale(drag_magnitude);
      forces.push(Force::new(String::from("Keel Lift"), self.loc, lift));
      forces.push(Force::new(String::from("Keel Drag"), self.loc, drag));
    }
    if self.vel.magnitude() != 0.0 || self.rot_vel != 0.0 {
      // Calculate angle of attack on the rudder
      let rudder_water_rot_vel = Vec2D::new(0.0, self.rot_vel * DELTA_TIME * HULL_LENGTH * 0.5).rotate(self.heading);
      let rudder_water_velocity = water_vel + rudder_water_rot_vel;
      let mut aoa: f64 = self.heading + self.rudder_angle - rudder_water_velocity.to_angle() ;
      if aoa < 0.0 {
        aoa = PI + aoa;
      }
      // Calculate rudder force vectors
      let lift_magnitude = calculate_lift(aoa, RUDDER_AREA, DENSITY_WATER, rudder_water_velocity.magnitude());
      let drag_magnitude = calculate_drag(aoa, RUDDER_AREA, DENSITY_WATER, rudder_water_velocity.magnitude());
      let lift = rudder_water_velocity.unit().rotate(PI/-2.0).scale(lift_magnitude);
      let drag = rudder_water_velocity.unit().scale(drag_magnitude);
      let rudder_offset = Vec2D::new(-HULL_LENGTH * 0.5, 0.0);
      let rudder_loc = rudder_offset.rotate(self.heading) + self.loc;
      forces.push(Force::new(String::from("Rudder Lift"), rudder_loc, lift));
      forces.push(Force::new(String::from("Rudder Drag"), rudder_loc, drag));

      // Calculate angle of attack on the front half of the hull
      let bow_water_rot_vel = Vec2D::new(0.0, -self.rot_vel * DELTA_TIME * HULL_LENGTH * 0.25).rotate(self.heading);
      let bow_water_vel = water_vel + bow_water_rot_vel;
      let mut aoa: f64 = self.heading - bow_water_vel.to_angle();
      if aoa < 0.0 {
        aoa = PI + aoa;
      }
      let apparent_width = f64::cos(aoa).abs() * HULL_WIDTH + f64::sin(aoa).abs() * HULL_LENGTH * 0.5;
      let wetted_area = HULL_DEPTH * apparent_width;
      let drag_magnitude = calculate_force(FRICTION_COEFFICIENT, wetted_area, DENSITY_WATER, bow_water_vel.magnitude());
      let drag = bow_water_vel.unit().scale(drag_magnitude);
      let offset = Vec2D::from_angle(self.heading).scale(HULL_LENGTH * 0.5);
      forces.push(Force::new(String::from("Hull Front Drag"), self.loc + offset, drag));

      // Calculate angle of attack on the rear half of the hull
      let stern_water_rot_vel = Vec2D::new(0.0, self.rot_vel * DELTA_TIME * HULL_LENGTH * 0.25).rotate(self.heading);
      let stern_water_vel = water_vel + stern_water_rot_vel;
      let mut aoa: f64 = self.heading - stern_water_vel.to_angle();
      if aoa < 0.0 {
        aoa = PI + aoa;
      }
      let apparent_width = f64::cos(aoa).abs() * HULL_WIDTH + f64::sin(aoa).abs() * HULL_LENGTH * 0.5;
      let wetted_area = HULL_DEPTH * apparent_width;
      let drag_magnitude = calculate_force(FRICTION_COEFFICIENT, wetted_area, DENSITY_WATER, stern_water_vel.magnitude());
      let drag = stern_water_vel.unit().scale(drag_magnitude);
      let offset = Vec2D::from_angle(self.heading).scale(HULL_LENGTH * 0.5);
      forces.push(Force::new(String::from("Hull Rear Drag"), self.loc - offset, drag));
    }
    return forces;
  }
}



#[tauri::command(rename_all = "snake_case")]
pub fn debug_ship_physics(wind_angle: f64, wind_speed: f64, velocity: Vec2D, rot_velocity: f64, heading: f64, sail: f64, rudder_angle: f64) -> PhysicsShapes {
  // Create the specified ship
  let mut ship = ForeAftRigShip::new(
    Vec2D::new(0.0,0.0),
    velocity,
    rot_velocity,
    heading,
    sail, // sail angle or mainsheet length
    rudder_angle
  );

  // Calculate all forces acting on the ship
  let mut forces = ship.forces(wind_angle, wind_speed);
  let wind_source: Vec2D = Vec2D::new(0.0, 13.0);
  let wind_vec: Vec2D = Vec2D::from_angle(invert_angle(wind_angle)).scale(wind_speed);
  let rot_source: Vec2D = Vec2D::new(13.0, 0.0);
  let rot_vec: Vec2D = Vec2D::new(0.0, rot_velocity);

  forces.insert(0, Force::new(String::from("Wind"), wind_source, wind_vec));
  forces.insert(1, Force::new(String::from("Velocity"), wind_source, velocity));
  forces.insert(2, Force::new(String::from("Apparent Wind"), wind_source, calculate_apparent_wind(velocity, wind_angle, wind_speed)));
  forces.insert(3, Force::new(String::from("Rotation"), rot_source, rot_vec));

  // forces.iter().for_each(|f| println!("{}", f.name));

  // Convert the forces and ship into drawable entities
  let mut arrows: Vec<Arrow> = Vec::new();
  forces.iter().for_each(|force|
    arrows.push(Arrow::from_force(force))
  );
  let default_ship = ForeAftRigShipShape::default(1.0);
  let shapes = PhysicsShapes {
    ship:ForeAftRigShipShape::new(&ship, &default_ship),
    forces: arrows
  };
  
  // DEBUG SAILING
  // ship.sail(wind_angle, wind_speed);

  return shapes;
}
