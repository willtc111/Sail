use std::{f64::consts::PI, sync::Mutex};

use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::geometry::Vec2D;
use crate::drawing::{Arrow, PhysicsShapes, ShipShape};

pub const DELTA_TIME: f64 = 1.0 / 30.0; // seconds

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct SimSettings {
  wind_angle: f64,
  wind_speed: f64,
}
impl SimSettings {
  pub fn new(wind_angle: f64, wind_speed: f64) -> Self {
    Self { wind_angle, wind_speed }
  }
}

pub struct Simulation {
  step: u64,
  population: Vec<Ship>,
  settings: SimSettings,
  random: StdRng,
}
impl Simulation {
  pub fn new(seed: u64, wind_angle: f64, wind_speed: f64) -> Self {
    Self {
      step: 0,
      population: Self::debug_ships(),
      settings: SimSettings::new(wind_angle, wind_speed),
      random: StdRng::seed_from_u64(seed)
    }
  }
  fn debug_ships() -> Vec<Ship> {
    let mut ships = Vec::new();
    ships.push(Ship::new(
      Vec2D::new(50.0, 25.0),
      Vec2D::new(0.0, 0.0),
      0.0,
      PI, -0.0, 0.0
    ));
    return ships;
  }
  pub fn step(&mut self) {
    self.step += 1;

    self.population.iter_mut().for_each(| ship |
      ship.sail(self.settings.wind_angle, self.settings.wind_speed)
    );
  }
  pub fn get_population(&self) -> &Vec<Ship> {
    return &self.population;
  }
  pub fn set_population(&mut self, population: Vec<Ship>) {
    self.population = population;
  }
  pub fn get_ship(&self, index: usize) -> Option<&Ship> {
    self.population.get(index)
  }
  pub fn update_ship_controls(&mut self, index: usize, sail: f64, rudder: f64) {
    match self.population.get_mut(index) {
      Some(ship) => {
        ship.sail_angle = sail;
        ship.rudder_angle = rudder;
      }
      None => {}
    }
  }
  pub fn reset(&mut self) {
    self.population = Self::debug_ships();
    self.step = 0;
    println!("reset sim");
  }
}


pub const HULL_WIDTH: f64 = 3.0;
pub const HULL_LENGTH: f64 = 10.0;
pub const HULL_DEPTH: f64 = 0.33;
pub const HULL_WETTED_AREA: f64 = HULL_WIDTH * HULL_LENGTH + 2.0 * HULL_DEPTH * (HULL_LENGTH + HULL_WIDTH);
pub const SAIL_WIDTH: f64 = 7.0;
pub const SAIL_HEIGHT: f64 = 10.0;
pub const SAIL_AREA: f64 = SAIL_WIDTH * SAIL_HEIGHT / 2.0; // half because triangle
pub const KEEL_LENGTH: f64 = 2.0;
pub const KEEL_HEIGHT: f64 = 1.0;
pub const KEEL_AREA: f64 = KEEL_LENGTH * KEEL_HEIGHT;
pub const RUDDER_LENGTH: f64 = 0.5;
pub const RUDDER_HEIGHT: f64 = 1.0;
pub const RUDDER_AREA: f64 = RUDDER_LENGTH * RUDDER_HEIGHT;

pub const HULL_DENSITY: f64 = 200.0; // kg / m^3, solid wood is ~600
pub const HULL_MASS: f64 = HULL_WIDTH * HULL_LENGTH * HULL_DENSITY;
pub const INVERSE_HULL_MASS: f64 = 1.0 / HULL_MASS;

pub const DENSITY_AIR: f64 = 1.225; // kg / m^3
pub const DENSITY_WATER: f64 = 1027.0; // kg / m^3

// found empirically? when water density is included in hull drag as the value that
//  provides a maximum speed of about 1.5 * wind speed
pub const FRICTION_COEFFICIENT: f64 = 0.007;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Force {
  pub name: String,
  pub loc: Vec2D,
  pub vec: Vec2D,
}
impl Force {
  pub fn new(name: String, loc: Vec2D, vec: Vec2D) -> Self {
    Self { name, loc, vec }
  }
}


#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Ship {
  pub loc: Vec2D,
  pub vel: Vec2D,
  pub rot_vel: f64,
  pub heading: f64,
  pub sail_angle: f64,
  pub rudder_angle: f64,
}
impl Ship {
  pub fn new(loc: Vec2D, vel: Vec2D, rot_vel: f64, heading: f64, sail_angle: f64, rudder_angle: f64) -> Self {
    Self { loc, vel, rot_vel, heading, sail_angle, rudder_angle }
  }

  pub fn sail(&mut self, wind_angle: f64, wind_speed: f64) {
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

  pub fn forces(&self, wind_angle: f64, wind_speed: f64) -> Vec<Force> {
    let mut forces = Vec::new();

    // Calculate angle of attack on sail
    let apparent_wind = self.apparent_wind(wind_angle, wind_speed).scale(DELTA_TIME);
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
      // Calculate hull drag
      // let apparent_width = f64::cos(aoa).abs() * HULL_WIDTH + f64::sin(aoa).abs() * HULL_LENGTH;
      // let hull_area = HULL_DEPTH * apparent_width;
      // let drag_magnitude = calculate_parasitic_drag(FRICTION_COEFFICIENT, hull_area, water_vel.magnitude());
      // let drag = water_vel.unit().scale(drag_magnitude);
      // forces.push(Force::new(String::from("Hull Drag"), self.loc, drag));
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

  pub fn apparent_wind(&self, wind_angle: f64, wind_speed: f64) -> Vec2D {
    let wind = Vec2D::from_angle(PI - wind_angle).scale(wind_speed);
    return wind - self.vel;
  }
}


fn bound_angle(angle: f64) -> f64 {
  let bounded = ((angle + PI).rem_euclid(2.0 * PI)) - PI;
  bounded
}

pub fn test_bound_angle() {
  let offsets = [0.0, -2.0*PI, 2.0*PI, -8.0*PI, 8.0*PI];
  for angle_deg in -180..180 {
    let mut angle_rad = f64::to_radians(f64::from(angle_deg));
    for offset in offsets {
      angle_rad = angle_rad + offset;
      let bounded_rad = bound_angle(angle_rad);
      let bounded_deg = f64::to_degrees(bounded_rad);
      assert_eq!(f64::from(angle_deg), bounded_deg.round());
    }
  }
}

fn calculate_lift(angle: f64, area: f64, density: f64, velocity: f64) -> f64 {
  let cl: f64 = calculate_lift_coefficient(angle);
  calculate_force(cl, area, density, velocity)
}

fn calculate_drag(angle: f64, area: f64, density: f64, velocity: f64) -> f64 {
  let cd: f64 = calculate_drag_coefficient(angle);
  calculate_force(cd, area, density, velocity)
}

fn calculate_force(coefficient: f64, area: f64, density: f64, velocity: f64) -> f64 {
  coefficient * area * density * velocity * velocity * 0.5
}

fn calculate_parasitic_drag(coefficient: f64, area: f64, velocity: f64) -> f64 {
  coefficient * area * velocity * velocity * 0.5
}

fn calculate_lift_coefficient(angle: f64) -> f64 {
  let cl: f64;
  // https://aviation.stackexchange.com/questions/64490/is-there-a-simple-relationship-between-angle-of-attack-and-lift-coefficient
  if (angle > 0.0 && angle < PI/8.0)
  || (angle > 7.0 * PI/8.0 && angle < PI) {
    cl = 1.1 * f64::sin(6.0 * angle);
  } else {
    cl = f64::sin(2.0 * angle);
  }
  return cl;
}

fn calculate_drag_coefficient(angle: f64) -> f64 {
  // https://aviation.stackexchange.com/questions/64490/is-there-a-simple-relationship-between-angle-of-attack-and-lift-coefficient
  let cd: f64 = 1.0 - f64::cos(2.0 * angle);
  return cd;
}

#[tauri::command]
pub fn debug_coefficients() -> (Vec<f64>, Vec<f64>) {
  let mut cls = Vec::new();
  let mut cds = Vec::new();
  for angle_deg in 0..180 {
    let angle_rad = f64::to_radians(f64::from(angle_deg));
    cls.push(calculate_lift_coefficient(angle_rad));
    cds.push(calculate_drag_coefficient(angle_rad));
  }
  return (cls, cds);
}

#[tauri::command]
pub fn reset_simulation(sim: State<Mutex<Simulation>>) {
  let mut sim = sim.lock().unwrap();

  sim.reset();
}

#[tauri::command]
pub fn step_simulation(sim: State<Mutex<Simulation>>) -> u64 {
  let mut sim = sim.lock().unwrap();

  sim.step();

  return sim.step;
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_sim_settings(sim: State<Mutex<Simulation>>) -> SimSettings {
  let mut sim = sim.lock().unwrap();
  return sim.settings;
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_sim_settings(sim: State<Mutex<Simulation>>, wind_angle: f64, wind_speed: f64) {
  let mut sim = sim.lock().unwrap();
  sim.settings.wind_angle = wind_angle;
  sim.settings.wind_speed = wind_speed;
}

#[tauri::command]
pub fn get_population(sim: State<Mutex<Simulation>>) -> Vec<ShipShape> {
  let sim = sim.lock().unwrap();

  let default_ship: ShipShape = ShipShape::default(1.0);
  let mut ships: Vec<ShipShape> = Vec::new();
  sim.get_population().iter().for_each(|ship| ships.push(ShipShape::new(ship, &default_ship)));
  return ships;
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_ship(sim: State<Mutex<Simulation>>, index: usize) -> Ship {
  let sim = sim.lock().unwrap();
  let ship = sim.get_ship(index);
  return *ship.unwrap();
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_ship_controls(sim: State<Mutex<Simulation>>, index: usize, sail_angle: f64, rudder_angle: f64) {
  let mut sim = sim.lock().unwrap();
  let ship = sim.update_ship_controls(index, sail_angle, rudder_angle);
}

#[tauri::command(rename_all = "snake_case")]
pub fn debug_physics(wind_angle: f64, wind_speed: f64, velocity: Vec2D, rot_velocity: f64, heading: f64, sail_angle: f64, rudder_angle: f64) -> PhysicsShapes {
  // Create the specified ship
  let mut ship = Ship::new(
    Vec2D::new(0.0,0.0),
    velocity,
    rot_velocity,
    heading,
    sail_angle,
    rudder_angle
  );

  // Calculate all forces acting on the ship
  let mut forces = ship.forces(wind_angle, wind_speed);
  let wind_source: Vec2D = Vec2D::new(0.0, 13.0);
  let wind_vec: Vec2D = Vec2D::from_angle(PI - wind_angle).scale(wind_speed);
  let rot_source: Vec2D = Vec2D::new(13.0, 0.0);
  let rot_vec: Vec2D = Vec2D::new(0.0, rot_velocity);

  forces.insert(0, Force::new(String::from("Wind"), wind_source, wind_vec));
  forces.insert(1, Force::new(String::from("Velocity"), wind_source, velocity));
  forces.insert(2, Force::new(String::from("Apparent Wind"), wind_source, ship.apparent_wind(wind_angle, wind_speed)));
  forces.insert(3, Force::new(String::from("Rotation"), rot_source, rot_vec));

  // forces.iter().for_each(|f| println!("{}", f.name));

  // Convert the forces and ship into drawable entities
  let mut arrows: Vec<Arrow> = Vec::new();
  forces.iter().for_each(|force|
    arrows.push(Arrow::from_force(force))
  );
  let default_ship: ShipShape = ShipShape::default(1.0);
  let shapes = PhysicsShapes {
    ship:ShipShape::new(&ship, &default_ship),
    forces: arrows
  };
  
  // DEBUG SAILING
  println!("");
  ship.sail(wind_angle, wind_speed);

  return shapes;
}
