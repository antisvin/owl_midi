extern crate bindgen;

use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=OpenWareMidiControl.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("OpenWareMidiControl.h")
        .prepend_enum_name(false)        
//        .fit_macro_constants(true)
        .rustified_enum("PatchButtonId")
        .rustified_enum("PatchParameterId")
        .rustified_enum("OpenWareMidiControl")
        .rustified_enum("OpenWareMidiSysexCommand")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let status = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("MidiStatus.h")
        .prepend_enum_name(false)  
//        .translate_enum_integer_types(true)
        .newtype_enum("MidiStatus")
        .newtype_enum("MidiControlChange")
        .newtype_enum("UsbMidi")
        .newtype_enum("OwlProtocol")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    status
        .write_to_file(out_path.join("status.rs"))
        .expect("Couldn't write status!");
}
