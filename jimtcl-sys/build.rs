extern crate bindgen;
extern crate cc;

use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    if !Path::new("jimtcl/.git").exists() {
        Command::new("git")
            .args(&["submodule", "update", "--init", "--recursive"])
            .status()
            .unwrap();
    }
    println!("cargo:rustc-link-search=./jimtcl");
    println!("cargo:rustc-link-lib=jim");
    let bindings = bindgen::Builder::default() // The input header we would like to generate         // bindings for.
        .header("wrapper.h") // Finish the builder and generate the bindings.
        .generate() // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings"); // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
