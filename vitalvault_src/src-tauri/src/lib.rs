use std::Option;
use std::sync::Mutex;

use tauri::{Builder, Manager};

use docstuff;

struct MockDBConn{} //TODO: replace with DuckDB connection type
struct MockMLEngine{} //TODO: replace Burn Backend reference

struct AppState{
  Option<MockDBConn> db
  bool has_auth = false
  Option<DocScope> scope
  Option<MockMLEngine> mlengine
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
