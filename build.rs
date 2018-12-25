use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    Command::new("make").args(&["-C", "./c/", "-f", "./Makefile"]).status().unwrap();

    println!("cargo:rustc-link-search=native=./c");
    println!("cargo:rustc-link-lib=dylib=mx");

    let bindings = bindgen::Builder::default()
        .header("./c/mx.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("c_mx_bindings.rs"))
        .expect("Couldn't write bindings!");

}
