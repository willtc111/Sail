// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use simulation::Simulation;
use std::sync::Mutex;

mod geometry;
mod simulation;
mod drawing;


fn main() {

  simulation::test_bound_angle();

  tauri::Builder::default()
  .manage(
    Mutex::new(Simulation::new(12345, 0.0, 5.0))
  )
  .invoke_handler(tauri::generate_handler![
    geometry::test_geometry,
    simulation::reset_simulation,
    simulation::step_simulation,
    simulation::get_population,
    simulation::debug_physics,
    simulation::debug_coefficients,
    simulation::get_sim_settings,
    simulation::set_sim_settings,
    simulation::get_ship,
    simulation::get_ship_id,
    simulation::set_ship_controls,
  ])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}
