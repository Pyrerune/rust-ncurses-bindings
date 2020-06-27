extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
	//Link to ncurses library
    println!("cargo:rustc-flags=-lncurses");
	//Have cargo rebuild if wrapper.h is changed
	println!("cargo:rerun-if-changed=wrapper.h");

    //bindgen options
    let bindings = bindgen::Builder::default()
        //wrapper.h contains the headers necessary for 
        //generating ncurses bindings
        .header("wrapper.h")
        //Generate the Bindings
        .generate()
        //Handle Errors
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/raw.rs")
        .expect("Couldn't write bindings!");
}
