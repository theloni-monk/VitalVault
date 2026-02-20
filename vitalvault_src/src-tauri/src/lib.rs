#![allow(unused)] // Disable dead code warnings for the entire crate
 
use std::option::Option;
use std::sync::Mutex;
use duckdb::{params, Connection, Result};
use tauri::{Builder, Manager};

use crate::docstruct::DocScope;

mod docstruct;
mod vvbackend;
mod mlpipeline;

struct MockMLEngine{} //TODO: replace Burn runtime Backend reference
struct MockAuthToken{} //TODO: replace with Stronghold auth token from native keychain via tauri-biometry
struct BackendState{
  db : Connection,
  auth_token: MockAuthToken,
  activescope : DocScope,
  mlengine: Option<MockMLEngine> 
}

//TODO: write tests

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        #[cfg(mobile)]
        app.handle().plugin(tauri_plugin_biometry::init());

        let salt_path = app
                .path()
                .app_local_data_dir()
                .expect("could not resolve app local data path")
                .join("vv.salt");
  
        app.handle()
          .plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;

        app.handle()
          .plugin(
            tauri_plugin_log::Builder::default()
              .level(log::LevelFilter::Info)
              .build()
          )?;

        app.handle()
          .plugin(tauri_plugin_fs::init())?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
