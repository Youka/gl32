use std::env;
use std::path::Path;
use std::fs::File;
use gl_generator::{Registry, Api, Profile, Fallbacks, StructGenerator};

fn main() {
    // Create target file for binding code
    let mut file = File::create(
        &Path::new(&env::var("OUT_DIR").expect("Build output directory should be known!")).join("bindings.rs")
    ).expect("Couldn't create temporary output file!");
    // Collect API information from Khronos registry and write code to file
    Registry::new(
        Api::Gl,    // OpenGL (Desktop)
        (3, 2), // Version 3.2
        Profile::Core,  // Just core / minimal contents
        Fallbacks::None,    // No backwards compatibility
        []  // No extensions
    ).write_bindings(StructGenerator, &mut file).expect("Couldn't write binding code to file!");
}