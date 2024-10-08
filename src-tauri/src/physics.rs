use std::f64::consts::PI;

use serde::Serialize;

use crate::geometry::{invert_angle, Vec2D};


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


pub fn calculate_aero_force_vecs(aoa: f64, area: f64, density: f64, velocity: Vec2D) -> (Vec2D, Vec2D) {
  let lift_magnitude = calculate_lift(aoa, area, density, velocity.magnitude());
  let drag_magnitude = calculate_drag(aoa, area, density, velocity.magnitude());
  let lift = velocity.unit().rotate(PI/-2.0).scale(lift_magnitude);
  let drag = velocity.unit().scale(drag_magnitude);
  return (lift, drag);
}


pub fn calculate_lift(angle: f64, area: f64, density: f64, velocity: f64) -> f64 {
  let cl: f64 = calculate_lift_coefficient(angle);
  return calculate_force(cl, area, density, velocity);
}

pub fn calculate_drag(angle: f64, area: f64, density: f64, velocity: f64) -> f64 {
  let cd: f64 = calculate_drag_coefficient(angle);
  return calculate_force(cd, area, density, velocity);
}

pub fn calculate_force(coefficient: f64, area: f64, density: f64, velocity: f64) -> f64 {
  coefficient * area * density * velocity * velocity * 0.5
}

// pub fn calculate_parasitic_drag(coefficient: f64, area: f64, velocity: f64) -> f64 {
//   coefficient * area * velocity * velocity * 0.5
// }

pub fn calculate_lift_coefficient(angle: f64) -> f64 {
  let cl: f64;
  // https://aviation.stackexchange.com/questions/64490/is-there-a-simple-relationship-between-angle-of-attack-and-lift-coefficient
  if (angle > 0.0 && angle < PI/8.0)
  || (angle < PI && angle > 7.0*PI/8.0) {
    cl = 1.1 * f64::sin(6.0 * angle);
  } else {
    cl = f64::sin(2.0 * angle);
  }
  return cl;
}

pub fn calculate_drag_coefficient(angle: f64) -> f64 {
  // https://aviation.stackexchange.com/questions/64490/is-there-a-simple-relationship-between-angle-of-attack-and-lift-coefficient
  let cd: f64 = 1.0 - f64::cos(2.0 * angle);
  return cd;
}

pub fn calculate_apparent_wind_simple(velocity: Vec2D, wind_angle: f64, wind_speed: f64) -> Vec2D {
  return calculate_apparent_wind(velocity, 0.0, 0.0, 0.0, wind_angle, wind_speed);
}

pub fn calculate_apparent_wind(velocity: Vec2D, rotational_velocity: f64, heading: f64, offset: f64, wind_angle: f64, wind_speed: f64) -> Vec2D {
  let wind = Vec2D::from_angle(invert_angle(wind_angle)).scale(wind_speed);
  // Assumes the rotational velocity across the whole sail is the same as it is at the mast, which is incorrect but simpler to implement
  let velocity_via_rotation = Vec2D::new(0.0, rotational_velocity * offset).rotate(heading);
  let total_velocity = velocity + velocity_via_rotation;
  // Subtract total velocity because movement creates apparent wind in the opposite direction
  return wind - total_velocity;
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