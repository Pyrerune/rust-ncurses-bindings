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
        //I've no idea what this does but the example told me to so I did
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        //Generate the Bindings
        .generate()
        //Handle Errors
        .expect("Unable to generate bindings");

    //Put the bindings in a place that is hard to reach because CARGO
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
