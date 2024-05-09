// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::{Rng, rngs::StdRng, SeedableRng};
use std::sync::Mutex;

fn main() {
  let seed:u64 = 123456; //123
  let mut random = StdRng::seed_from_u64(seed);

  tauri::Builder::default()
  .manage(
    Mutex::new(random)
  )
  .invoke_handler(tauri::generate_handler![
  ])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}
