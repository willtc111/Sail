// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use simulation::Simulation;
use std::sync::Mutex;

mod geometry;
mod simulation;
mod drawing;
mod physics;
mod ship;


fn main() {
  tauri::Builder::default()
  .manage(
    Mutex::new(Simulation::new(12345, 0.0, 5.0))
  )
  .invoke_handler(tauri::generate_handler![
    simulation::reset_simulation,
    simulation::step_simulation,
    simulation::get_population,
    simulation::get_sim_settings,
    simulation::set_sim_settings,
    simulation::get_ship,
    simulation::get_ship_id,
    simulation::set_ship_controls,
    ship::debug_ship_physics,
    physics::debug_coefficients,
  ])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}
