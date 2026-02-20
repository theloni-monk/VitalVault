fn main() {
    // Generate TypeScript bindings before build
    tauri_typegen::BuildSystem::generate_at_build_time()
        .expect("Failed to generate TypeScript bindings");

    println!("cargo:rustc-link-search=native=C:\\Windows\\System32");
    println!("cargo:rustc-link-lib=Rstrtmgr");

    tauri_build::build()
}