use std::{f64::consts::PI, sync::Mutex};

use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::geometry::Vec2D;
use crate::drawing::SquareRigShipShape;
use crate::ship::{Ship, SquareRigShip, HULL_LENGTH};

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
  population: Vec<SquareRigShip>,
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
  fn debug_ships() -> Vec<SquareRigShip> {
    let mut ships = Vec::new();
    ships.push(SquareRigShip::new(
      Vec2D::new(50.0, 25.0),
      Vec2D::new(0.0, 0.0),
      0.0,
      PI, -0.0, 0.0
    ));
    ships.push(SquareRigShip::new(
      Vec2D::new(50.0, 50.0),
      Vec2D::new(0.0, 0.0),
      0.0,
      PI, -0.0, 0.0
    ));
    ships.push(SquareRigShip::new(
      Vec2D::new(50.0, 75.0),
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
  pub fn get_population(&self) -> &Vec<SquareRigShip> {
    return &self.population;
  }
  pub fn set_population(&mut self, population: Vec<SquareRigShip>) {
    self.population = population;
  }
  pub fn get_ship(&self, index: usize) -> Option<&SquareRigShip> {
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
  let sim = sim.lock().unwrap();
  return sim.settings;
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_sim_settings(sim: State<Mutex<Simulation>>, wind_angle: f64, wind_speed: f64) {
  let mut sim = sim.lock().unwrap();
  sim.settings.wind_angle = wind_angle;
  sim.settings.wind_speed = wind_speed;
}

#[tauri::command]
pub fn get_population(sim: State<Mutex<Simulation>>) -> Vec<SquareRigShipShape> {
  let sim = sim.lock().unwrap();

  let default_ship: SquareRigShipShape = SquareRigShipShape::default(1.0);
  let mut ships: Vec<SquareRigShipShape> = Vec::new();
  sim.get_population().iter().for_each(|ship| ships.push(SquareRigShipShape::new(ship, &default_ship)));
  return ships;
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_ship(sim: State<Mutex<Simulation>>, index: usize) -> SquareRigShip {
  let sim = sim.lock().unwrap();
  let ship = sim.get_ship(index);
  return *ship.unwrap();
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_ship_id(sim: State<Mutex<Simulation>>, loc: Vec2D) -> Option<usize> {
  let sim = sim.lock().unwrap();
  for i in 0..sim.get_population().len() {
    let ship = sim.get_population()[i];
    if ship.loc.dist(loc) < HULL_LENGTH / 2.0 {
      return Some(i);
    }
  }
  return Option::None;
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_ship_controls(sim: State<Mutex<Simulation>>, index: usize, sail_angle: f64, rudder_angle: f64) {
  let mut sim = sim.lock().unwrap();
  sim.update_ship_controls(index, sail_angle, rudder_angle);
}
