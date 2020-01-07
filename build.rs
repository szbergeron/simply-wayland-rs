extern crate bindgen;

use std::env;
use std::path::PathBuf;
//use std::fs::OpenOptions;
//use std::io::prelude::*;
//use regex::Regex;
//use onig::*;

fn main() {
    println!("cargo:rustc-link-lib=wayland-client");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=path/to/Cargo.lock");
    println!("cargo:rerun-if-changed=pipe.sh");

    let mut stripped_path = std::env::var("OUT_DIR").unwrap();
    stripped_path.push_str("/stripped.h");

    std::process::Command::new("sh")
        .arg("-c")
        .arg("clang wrapper.h -c -Dstatic= -Dinline= -D__inline= -D__inline__= -E -P > $OUT_DIR/output.h")
        .output()
        .expect("Couldn't generate un-inlined header");

    println!("EVENT: compiled wrapper into output");

    std::process::Command::new("sh")
        .arg("./pipe.sh")
        .output()
        .expect("Couldn't remove bad typedefs");

    println!("EVENT: removed bad typedefs");

    // removing wlinline for now as dedicated rust wrappers are more ergonomic
    /*cc::Build::new()
        .file(stripped_path.as_str())
        .compile("wlinline");*/

    println!("EVENT: compiled lib");

    let bindings = bindgen::Builder::default()
        .header(stripped_path.as_str())
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
        .generate_inline_functions(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings");

    //std::fs::remove_file("output.h").unwrap();
    //std::fs::remove_file("stripped.h").unwrap();

    //let mut link_loc_directive: String = "cargo:rustc-link-search=native=".to_owned();
    //link_loc_directive.push_str(env::var("OUT_DIR").unwrap().as_str());
    //println!("cargo:rustc-link-search=native={}/wlinline.a", env::var("OUT_DIR").unwrap());
    //println!("cargo:rustc-link-lib=static=wlinline");
    //println!(&(String::from("cargo:rustc-link-search=native=") + env::var("OUT_DIR").unwrap())[..]);

    //println!("cargo:rustc-link-lib-static=

    // copy inline functions to separate file to use c2rust
    // match a line starting with `static`, a line starting with `wl_` (function name),
    // and then a single parent open/close brace pair with recursive inner pairs
    // (static)(^[\n|\r\n|\r])*[\n|\r\n|\r]
    // \(([^()]|(?R))*\)
    
    /*let contents = std::fs::read_to_string("/usr/include/wayland-client-protocol.h").expect("Couldn't read protocol header");
    //println!("{}", &contents);

    //let regex = "(static)([\\s\\w\\*\\(\\)]*)\\{([^\\{\\}]|(?R))*\\}";
    let re = Regex::new(r"(static[\s\w\*\(\)]*(?<rec>\{([^\{\}]|(\g<rec>))*\}))").unwrap();
    let captures = re.find_iter(&contents);
    //println!("{:?}", captures);
    std::fs::remove_file("inline.c").unwrap();
    std::fs::File::create("inline.c").unwrap();
    let file = OpenOptions::new().append(true).open("inline.c").unwrap();
    let mut buffer = std::io::BufWriter::new(file);

    //buffer.write("#include <wayland-client.h>\n#include <stdint.h>\n".as_bytes());
    buffer.write("#include <stdint.h>\n\n".as_bytes()).unwrap();
    for (start, end) in captures {
        //
        //std::fs::write("inline.c", capture.unwrap());
        //writeln!(file, capture.unwrap());
        //buffer.write(capture.unwrap().as_bytes()).unwrap();
        let string = &contents[start..end];
        //let start = capture.next();
        //let start = 
        //buffer.write("//starts: {}"
        println!("starts: {}", start);
        println!("string: {}", string);
        buffer.write(string.as_bytes()).unwrap();
        //buffer.write
        buffer.write("\n\n".as_bytes()).unwrap();
        println!("ends: {}", end);
    }
    buffer.write("//end".as_bytes()).unwrap();
    buffer.flush().unwrap();
    //panic!("sdfjksdfalk");
    //std::fs::write("inline.c", 
    //panic!("djfskl");
    */
}
