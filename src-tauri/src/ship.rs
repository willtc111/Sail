use std::f64::consts::PI;

use serde::Serialize;

use crate::{
  drawing::{Arrow, ForeAftRigShipShape, PhysicsShapes},
  geometry::{bound, bound_angle, find_angle, invert_angle, Vec2D},
  physics::{calculate_aero_force_vecs, calculate_apparent_wind, calculate_apparent_wind_simple, calculate_force, Force},
  simulation::DELTA_TIME
};


pub const HULL_WIDTH: f64 = 3.0;
pub const HULL_LENGTH: f64 = 10.0;
pub const HULL_DEPTH: f64 = 0.33;

pub const HULL_THICKNESS: f64 = 0.05; // 5 cm thick
pub const HULL_VOLUME: f64 = HULL_THICKNESS * (HULL_WIDTH * HULL_LENGTH + 2.0 * HULL_WIDTH * HULL_DEPTH + 2.0 * HULL_LENGTH * HULL_DEPTH);
pub const HULL_MASS: f64 = HULL_VOLUME * DENSITY_WOOD;
pub const INVERSE_HULL_MASS: f64 = 1.0 / HULL_MASS;
// Determined empirically so that the maximum speed is roughly 2x wind speed
pub const HULL_FRICTION_COEFFICIENT: f64 = 0.007;

pub const MAST_OFFSET: f64 = 4.0;
pub const SAIL_WIDTH: f64 = 7.0;
pub const SAIL_HEIGHT: f64 = 10.0;
pub const SAIL_AREA: f64 = SAIL_WIDTH * SAIL_HEIGHT / 2.0; // half because triangle
pub const SAIL_AERO_CENTER: f64 = 0.33; // Arbitrarily picked 1/3 of the width from the mast

pub const KEEL_START_OFFSET: f64 = 1.0;
pub const KEEL_LENGTH: f64 = 2.0;
pub const KEEL_HEIGHT: f64 = 2.0;

pub const RUDDER_LENGTH: f64 = 1.0;
pub const RUDDER_HEIGHT: f64 = 1.0;
pub const RUDDER_AREA: f64 = RUDDER_LENGTH * RUDDER_HEIGHT;

pub const DENSITY_AIR: f64 = 1.225; // kg / m^3
pub const DENSITY_WATER: f64 = 1027.0; // kg / m^3
pub const DENSITY_WOOD: f64 = 750.0; // kg / m^3, solid white oak
pub const DENSITY_SAIL: f64 = 0.237; // kg / m^2 canvas (7 oz/yd^2)


pub trait Ship {
  /// Update the physical state of the ship
  fn update(&mut self, wind_angle: f64, wind_speed: f64);

  /// Calculate all of the forces acting on the ship
  fn forces(&mut self, wind_angle: f64, wind_speed: f64) -> Vec<Force>;
}

pub struct SailSpecs {
  pub mast_offset: f64,
  pub width: f64,
  pub height: f64,
}
impl SailSpecs {
  pub fn new(mast_offset: f64, width: f64, height: f64) -> Self {
    Self { mast_offset, width, height }
  }
}

pub struct ShipSpecs {
  pub hull_width: f64,
  pub hull_length: f64,
  pub hull_depth: f64,
  pub hull_thickness: f64,
  pub keel_start_offset: f64,
  pub keel_length: f64,
  pub keel_height: f64,
  pub rudder_length: f64,
  pub rudder_height: f64,
  pub sails: Vec<SailSpecs>,
}
impl ShipSpecs {
  pub fn new(hull_width: f64, hull_length: f64, hull_depth: f64, hull_thickness: f64,
    keel_start_offset: f64, keel_length: f64, keel_height: f64,
    rudder_length: f64, rudder_height: f64, sails: Vec<SailSpecs>) -> Self {
    Self {
      hull_width, hull_length, hull_depth, hull_thickness,
      keel_start_offset, keel_length, keel_height,
      rudder_length, rudder_height, sails
    }
  }
  pub fn default() -> Self {
    Self::new(
      3.0, 10.0, 0.33, 0.05,
      1.0, 2.0, 2.0,
      1.0, 1.0, vec![SailSpecs::new(4.0, 7.0, 10.0)]
    )
  }

  pub fn validate(&self) -> Result<(), String> {
    // All dimensions are positive, non-zero values
    if self.hull_width > 0.0 && self.hull_length > 0.0 && self.hull_depth > 0.0
        && self.keel_length > 0.0 && self.keel_height > 0.0
        && self.rudder_length > 0.0 && self.rudder_height > 0.0
        && self.sails.iter().all(|s| s.width > 0.0 && s.height > 0.0) {
      return Result::Err(String::from("Ship dimensions must all be greater than zero"));
    }

    // Keel fits on the hull
    let half_hull_length = self.hull_length / 2.0;
    if self.keel_start_offset < half_hull_length
        && self.keel_start_offset - self.keel_length > -half_hull_length {
      return Result::Err(String::from("Keel is outside the hull footprint"));
    }

    // Sails are sorted front to back and do not overlap
    let mut sail_start_limit = half_hull_length;
    for sail_index in 0..self.sails.len() {
      let sail = &self.sails[sail_index];
      let sail_start = sail.mast_offset;
      let sail_end = sail.mast_offset - sail.width;
      if sail_start >= sail_start_limit {
        return Result::Err(format!("Sail {} is too far forward or sails are not in order", sail_index));
      }
      let sail_end_limit = self.sails.get(sail_index+1).map_or_else(|| -half_hull_length, |s| s.mast_offset);
      if sail_end <= sail_end_limit {
        return Result::Err(format!("Sail {} extends too far aft or sails are not in order", sail_index));
      }
      sail_start_limit = sail_end;
    }

    return Result::Ok(());
  }
}

pub struct AdjustableShip {
  // Specifications of the ship
  pub specs: ShipSpecs,

  // Indirectly controlled state
  pub loc: Vec2D,
  pub vel: Vec2D,
  pub rot_vel: f64,
  pub heading: f64,
  pub sail_angles: Vec<f64>,

  // Directly controlled state
  pub mainsheet_lengths: Vec<f64>,
  pub rudder_angle: f64,
}
impl AdjustableShip {
  pub fn new(specs: ShipSpecs, loc: Vec2D, vel: Vec2D, rot_vel: f64, heading: f64, rudder_angle: f64) -> Self {
    let sail_angles: Vec<f64> = vec![0.0; specs.sails.len()];
    let mainsheet_lengths: Vec<f64> = vec![0.0; specs.sails.len()];
    Self { specs, loc, vel, rot_vel, heading, mainsheet_lengths, sail_angles, rudder_angle }
  }

  /// Calculate the angle the sail should be based on the apparent wind angle
  fn set_sail_angle(&mut self, sail_index: usize, apparent_wind_angle: f64) -> f64{
    let sail_spec = &self.specs.sails[sail_index];
    let max_sail_angle = find_angle(sail_spec.width, sail_spec.width, self.mainsheet_lengths[sail_index]);
    let hull_relative_apparent_wind_angle = bound_angle(invert_angle(apparent_wind_angle) - self.heading);
    let sail_angle = match hull_relative_apparent_wind_angle.abs() <= max_sail_angle {
      // Sail is luffing
      true => hull_relative_apparent_wind_angle,
      // Sail is taut, and stays leeward until forced to switch sides
      false => max_sail_angle * bound_angle(hull_relative_apparent_wind_angle - self.sail_angles[sail_index]).signum()
    };
    self.sail_angles[sail_index] = sail_angle;
    return sail_angle;
  }
}
impl Ship for AdjustableShip {
  fn update(&mut self, wind_angle: f64, wind_speed: f64) {
    let mass_hull = DENSITY_WOOD * self.specs.hull_thickness * (
      self.specs.hull_length * self.specs.hull_width // Bottom
      + 2.0 * self.specs.hull_width * self.specs.hull_depth // Two Sides
      + 2.0 * self.specs.hull_length * self.specs.hull_depth // Front & Back
    );
    let mass_sails: f64 = self.specs.sails.iter().map(|sail| {
      let sail_mass = DENSITY_SAIL * sail.width * sail.height * 0.5; // Half because triangle
      let sail_thickness = 0.01 * sail.width * sail.height; // Guessing a mast needs to be about 1 cm thick for every square meter of sail
      let mast_mass = DENSITY_WOOD * sail.height * sail_thickness * sail_thickness;
      return sail_mass + mast_mass;
    }).sum();
    let inverse_mass = 1.0 / (mass_hull + mass_sails);

    self.forces(wind_angle, wind_speed).iter().for_each(|force| {
      // A force always changes the velocity
      self.vel = self.vel + force.vec.scale(inverse_mass);

      // Rotate force vector so that x component is in line with center of mass
      let offset = force.loc - self.loc;
      let loc_rot = offset.rotate(-offset.to_angle());
      let vec_rot = force.vec.rotate(-offset.to_angle());
      // Then only the vector's y component contributes to rotation
      let torque = vec_rot.y * loc_rot.magnitude();
      // Just assume the ship is a point mass even though it obviously isn't
      self.rot_vel = self.rot_vel + torque * inverse_mass;
    });

    self.loc = self.loc + self.vel.scale(DELTA_TIME);
    self.heading = bound_angle(self.heading + self.rot_vel * DELTA_TIME);
  }

  fn forces(&mut self, wind_angle: f64, wind_speed: f64) -> Vec<Force> {
    let mut forces = Vec::new();
    // Sail forces
    if wind_speed != 0.0 || self.vel.magnitude() != 0.0 || self.rot_vel != 0.0 {
      for sail_index in 0..self.specs.sails.len() {
        let sail = &self.specs.sails[sail_index];
        let sail_area = sail.width * sail.height * 0.5; // Half because triangle
        let apparent_wind = calculate_apparent_wind(
          self.vel, self.rot_vel, self.heading, sail.mast_offset,
          wind_angle, wind_speed
        ).scale(DELTA_TIME);
        let apparent_wind_angle = apparent_wind.to_angle();
        let sail_angle = self.set_sail_angle(sail_index, apparent_wind_angle);
        let aoa = bound(self.heading + sail_angle - apparent_wind_angle, 0.0, PI);
        let (lift, drag) = calculate_aero_force_vecs(aoa, sail_area, DENSITY_AIR, apparent_wind);
        let sail_center = self.loc
          + Vec2D::new(MAST_OFFSET, 0.0).rotate(self.heading)
          + Vec2D::new(-SAIL_WIDTH*SAIL_AERO_CENTER, 0.0).rotate(self.heading + sail_angle);
        forces.push(Force::new(String::from(format!("Sail {} Lift", sail_index)), sail_center, lift));
        forces.push(Force::new(String::from(format!("Sail {} Drag", sail_index)), sail_center, drag));
      }
    }

    let water_vel = self.vel.scale(-1.0 * DELTA_TIME);
    if self.vel.magnitude() != 0.0 || self.rot_vel != 0.0 {
      // Calculate keel forces, split between the front and back portions of the keel to account for different rotations
      let (fore_keel_length, aft_keel_length) =
        if self.specs.keel_start_offset <= 0.0 {
          (0.0, self.specs.keel_length)
        } else {
          let aft = f64::max(0.0, self.specs.keel_length - self.specs.keel_start_offset);
          (self.specs.keel_length - aft, aft)
        };

      if fore_keel_length > 0.0 {
        let keel_center = self.specs.keel_start_offset - fore_keel_length*0.5;
        let water_rot_vel = Vec2D::new(0.0, -self.rot_vel * DELTA_TIME * keel_center).rotate(self.heading);
        let rel_water_vel = water_vel + water_rot_vel;
        let aoa: f64 = bound(self.heading - rel_water_vel.to_angle(), 0.0, PI);
        let (lift, drag) = calculate_aero_force_vecs(aoa, self.specs.keel_height*fore_keel_length, DENSITY_WATER, rel_water_vel);
        let keel_loc = Vec2D::new(keel_center, 0.0).rotate(self.heading) + self.loc;
        forces.push(Force::new(String::from("Fore Keel Lift"), keel_loc, lift));
        forces.push(Force::new(String::from("Fore Keel Drag"), keel_loc, drag));
      }

      if aft_keel_length > 0.0 {
        let keel_end_offset = self.specs.keel_start_offset - self.specs.keel_length;
        let keel_center = keel_end_offset + aft_keel_length*0.5;
        let water_rot_vel = Vec2D::new(0.0, -self.rot_vel * DELTA_TIME * keel_center).rotate(self.heading);
        let rel_water_vel = water_vel + water_rot_vel;
        let aoa: f64 = bound(self.heading - rel_water_vel.to_angle(), 0.0, PI);
        let (lift, drag) = calculate_aero_force_vecs(aoa, self.specs.keel_height*aft_keel_length, DENSITY_WATER, rel_water_vel);
        let keel_loc = Vec2D::new(keel_center, 0.0).rotate(self.heading) + self.loc;
        forces.push(Force::new(String::from("Aft Keel Lift"), keel_loc, lift));
        forces.push(Force::new(String::from("Aft Keel Drag"), keel_loc, drag));
      }

      // Calculate rudder forces
      let water_rot_vel = Vec2D::new(0.0, self.rot_vel * DELTA_TIME * self.specs.hull_length * 0.5).rotate(self.heading);
      let rel_water_vel = water_vel + water_rot_vel;
      let aoa: f64 = bound(self.heading + self.rudder_angle - rel_water_vel.to_angle(), 0.0, PI);
      let (lift, drag) = calculate_aero_force_vecs(aoa, self.specs.rudder_height * self.specs.rudder_length, DENSITY_WATER, rel_water_vel);
      let rudder_loc = Vec2D::new(-self.specs.hull_length * 0.5, 0.0).rotate(self.heading) + self.loc;
      forces.push(Force::new(String::from("Rudder Lift"), rudder_loc, lift));
      forces.push(Force::new(String::from("Rudder Drag"), rudder_loc, drag));

      // Calculate hull drag forces
      let bow_water_rot_vel = Vec2D::new(0.0, -self.rot_vel * DELTA_TIME * self.specs.hull_length * 0.25).rotate(self.heading);
      let bow_water_vel = water_vel + bow_water_rot_vel;
      let aoa: f64 = bound(self.heading - bow_water_vel.to_angle(), 0.0, PI);
      let apparent_width = f64::cos(aoa).abs() * self.specs.hull_width + f64::sin(aoa).abs() * self.specs.hull_length * 0.5;
      let wetted_area = HULL_DEPTH * apparent_width;
      let drag_magnitude = calculate_force(HULL_FRICTION_COEFFICIENT, wetted_area, DENSITY_WATER, bow_water_vel.magnitude());
      let drag = bow_water_vel.unit().scale(drag_magnitude);
      let offset = Vec2D::from_angle(self.heading).scale(self.specs.hull_length * 0.5);
      forces.push(Force::new(String::from("Bow Drag"), self.loc + offset, drag));

      let stern_water_rot_vel = Vec2D::new(0.0, self.rot_vel * DELTA_TIME * self.specs.hull_length * 0.25).rotate(self.heading);
      let stern_water_vel = water_vel + stern_water_rot_vel;
      let aoa: f64 = bound(self.heading - stern_water_vel.to_angle(), 0.0, PI);
      let apparent_width = f64::cos(aoa).abs() * self.specs.hull_width + f64::sin(aoa).abs() * self.specs.hull_length * 0.5;
      let wetted_area = HULL_DEPTH * apparent_width;
      let drag_magnitude = calculate_force(HULL_FRICTION_COEFFICIENT, wetted_area, DENSITY_WATER, stern_water_vel.magnitude());
      let drag = stern_water_vel.unit().scale(drag_magnitude);
      let offset = Vec2D::from_angle(self.heading).scale(self.specs.hull_length * 0.5);
      forces.push(Force::new(String::from("Stern Drag"), self.loc - offset, drag));
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

  /// Calculate the angle the sail should be based on the apparent wind angle
  fn set_sail_angle(&mut self, apparent_wind_angle: f64) {
    let max_sail_angle = find_angle(SAIL_WIDTH, SAIL_WIDTH, self.mainsheet_length);

    let hull_relative_apparent_wind_angle = bound_angle(invert_angle(apparent_wind_angle) - self.heading);
    self.sail_angle = if hull_relative_apparent_wind_angle.abs() <= max_sail_angle {
      // Sail is luffing
      hull_relative_apparent_wind_angle
    } else {
      // Sail is taut, and stays on the side of the wind that it is already on
      max_sail_angle * bound_angle(hull_relative_apparent_wind_angle - self.sail_angle).signum()
    };
  }
}
impl Ship for ForeAftRigShip {
  fn update(&mut self, wind_angle: f64, wind_speed: f64) {
    let f = self.forces(wind_angle, wind_speed);
    f.iter().for_each(|force| {
      // Force always changes the velocity
      self.vel = self.vel + force.vec.scale(INVERSE_HULL_MASS);

      if force.loc != self.loc {
        // Rotate force vector so that x component is in line with center of mass
        let offset = force.loc - self.loc;
        let loc_rot = offset.rotate(-offset.to_angle());
        let vec_rot = force.vec.rotate(-offset.to_angle());
        // Only vector y component contributes to rotation
        let torque = vec_rot.y * loc_rot.magnitude();
        self.rot_vel = self.rot_vel + torque * INVERSE_HULL_MASS;
      }
    });

    self.loc = self.loc + self.vel.scale(DELTA_TIME);
    self.heading = bound_angle(self.heading + self.rot_vel * DELTA_TIME);
  }

  fn forces(&mut self, wind_angle: f64, wind_speed: f64) -> Vec<Force> {
    let mut forces = Vec::new();

    let apparent_wind = calculate_apparent_wind_simple(self.vel, wind_angle, wind_speed).scale(DELTA_TIME);
    if apparent_wind.magnitude() != 0.0 {
      let apparent_wind_angle = apparent_wind.to_angle();

      self.set_sail_angle(apparent_wind_angle);

      // Calculate sail forces
      let aoa = bound(self.heading + self.sail_angle - apparent_wind_angle, 0.0, PI);
      let (lift, drag) = calculate_aero_force_vecs(aoa, SAIL_AREA, DENSITY_AIR, apparent_wind);
      let sail_center = self.loc + Vec2D::new(MAST_OFFSET, 0.0).rotate(self.heading) + Vec2D::new(-SAIL_WIDTH*SAIL_AERO_CENTER, 0.0).rotate(self.heading + self.sail_angle);
      forces.push(Force::new(String::from("Sail Lift"), sail_center, lift));
      forces.push(Force::new(String::from("Sail Drag"), sail_center, drag));
    }

    let water_vel = self.vel.scale(-1.0 * DELTA_TIME);
    if self.vel.magnitude() != 0.0 || self.rot_vel != 0.0 {

      // Calculate keel forces
      let (fore_keel_length, aft_keel_length) =
        if KEEL_START_OFFSET <= 0.0 {
          (0.0, KEEL_LENGTH)
        } else {
          let aft = f64::max(0.0, KEEL_LENGTH - KEEL_START_OFFSET);
          (KEEL_LENGTH - aft, aft)
        };

      if fore_keel_length > 0.0 {
        let fk_center = KEEL_START_OFFSET - fore_keel_length*0.5;
        let water_rot_vel = Vec2D::new(0.0, -self.rot_vel * DELTA_TIME * fk_center).rotate(self.heading);
        let rel_water_vel = water_vel + water_rot_vel;
        let aoa: f64 = bound(self.heading - rel_water_vel.to_angle(), 0.0, PI);
        let (lift, drag) = calculate_aero_force_vecs(aoa, KEEL_HEIGHT*fore_keel_length, DENSITY_WATER, rel_water_vel);
        let keel_offset = Vec2D::new(fk_center, 0.0);
        let keel_loc = keel_offset.rotate(self.heading) + self.loc;
        forces.push(Force::new(String::from("Fore Keel Lift"), keel_loc, lift));
        forces.push(Force::new(String::from("Fore Keel Drag"), keel_loc, drag));
      }

      if aft_keel_length > 0.0 {
        let keel_end_offset = KEEL_START_OFFSET - KEEL_LENGTH;
        let ak_center = keel_end_offset + aft_keel_length*0.5;
        let water_rot_vel = Vec2D::new(0.0, -self.rot_vel * DELTA_TIME * ak_center).rotate(self.heading);
        let rel_water_vel = water_vel + water_rot_vel;
        let aoa: f64 = bound(self.heading - rel_water_vel.to_angle(), 0.0, PI);
        let (lift, drag) = calculate_aero_force_vecs(aoa, KEEL_HEIGHT*aft_keel_length, DENSITY_WATER, rel_water_vel);
        let keel_offset = Vec2D::new(ak_center, 0.0);
        let keel_loc = keel_offset.rotate(self.heading) + self.loc;
        forces.push(Force::new(String::from("Aft Keel Lift"), keel_loc, lift));
        forces.push(Force::new(String::from("Aft Keel Drag"), keel_loc, drag));
      }

      // Calculate rudder forces
      let water_rot_vel = Vec2D::new(0.0, self.rot_vel * DELTA_TIME * HULL_LENGTH * 0.5).rotate(self.heading);
      let rel_water_vel = water_vel + water_rot_vel;
      let aoa: f64 = bound(self.heading + self.rudder_angle - rel_water_vel.to_angle(), 0.0, PI);
      let (lift, drag) = calculate_aero_force_vecs(aoa, RUDDER_AREA, DENSITY_WATER, rel_water_vel);
      let rudder_offset = Vec2D::new(-HULL_LENGTH * 0.5, 0.0);
      let rudder_loc = rudder_offset.rotate(self.heading) + self.loc;
      forces.push(Force::new(String::from("Rudder Lift"), rudder_loc, lift));
      forces.push(Force::new(String::from("Rudder Drag"), rudder_loc, drag));

      // Calculate hull drag forces
      let bow_water_rot_vel = Vec2D::new(0.0, -self.rot_vel * DELTA_TIME * HULL_LENGTH * 0.25).rotate(self.heading);
      let bow_water_vel = water_vel + bow_water_rot_vel;
      let aoa: f64 = bound(self.heading - bow_water_vel.to_angle(), 0.0, PI);
      let apparent_width = f64::cos(aoa).abs() * HULL_WIDTH + f64::sin(aoa).abs() * HULL_LENGTH * 0.5;
      let wetted_area = HULL_DEPTH * apparent_width;
      let drag_magnitude = calculate_force(HULL_FRICTION_COEFFICIENT, wetted_area, DENSITY_WATER, bow_water_vel.magnitude());
      let drag = bow_water_vel.unit().scale(drag_magnitude);
      let offset = Vec2D::from_angle(self.heading).scale(HULL_LENGTH * 0.5);
      forces.push(Force::new(String::from("Hull Front Drag"), self.loc + offset, drag));

      let stern_water_rot_vel = Vec2D::new(0.0, self.rot_vel * DELTA_TIME * HULL_LENGTH * 0.25).rotate(self.heading);
      let stern_water_vel = water_vel + stern_water_rot_vel;
      let aoa: f64 = bound(self.heading - stern_water_vel.to_angle(), 0.0, PI);
      let apparent_width = f64::cos(aoa).abs() * HULL_WIDTH + f64::sin(aoa).abs() * HULL_LENGTH * 0.5;
      let wetted_area = HULL_DEPTH * apparent_width;
      let drag_magnitude = calculate_force(HULL_FRICTION_COEFFICIENT, wetted_area, DENSITY_WATER, stern_water_vel.magnitude());
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
  forces.insert(2, Force::new(String::from("Apparent Wind"), wind_source, calculate_apparent_wind_simple(velocity, wind_angle, wind_speed)));
  forces.insert(3, Force::new(String::from("Rotation"), rot_source, rot_vec));

  // Debug list of forces
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

  // Debug application of forces
  ship.update(wind_angle, wind_speed);

  return shapes;
}
