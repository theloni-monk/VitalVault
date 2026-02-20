// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tokio;

#[tokio::main]
async fn main() {
  vitalvault_lib::run();
}

//TODO: correct all the manifest stuff with branding (icons, resources, etc)