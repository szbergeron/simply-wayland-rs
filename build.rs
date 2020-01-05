extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=wayland-client");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_type("wl.*")
        .whitelist_function("wl.*")
        .whitelist_var("wl.*")
        .whitelist_type(".*WL.*")
        .rustified_enum("wl_display_error")
        .rustified_enum("wl_shm_.*") // WARN: odd vals, possibly bitmap. Reeval later
        .rustified_enum("wl_data_.*")
        .rustified_enum("wl_shell_.*")
        .rustified_enum("wl_surface_.*")
        .rustified_enum("wl_seat_.*")
        .rustified_enum("wl_pointer_.*")
        .rustified_enum("wl_keyboard_.*")
        .rustified_enum("wl_output_.*")
        .rustified_enum("wl_subcompositor_.*")
        .rustified_enum("wl_subsurface_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings");
}
