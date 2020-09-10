extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    if cfg!(any(target_os = "macos", target_os = "ios")) {
        // The bindgen::Builder is the main entry point
        // to bindgen, and lets you build up options for
        // the resulting bindings.
        let bindings = bindgen::Builder::default()
            // The input header we would like to generate
            // bindings for.
            .header("./src/mach/mach_wrapper.h")
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .whitelist_function("mach_thread_self")
            .whitelist_function("thread_info")
            .whitelist_var("THREAD_BASIC_INFO_COUNT")
            .whitelist_type("thread_basic_info_data_t")
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("./mach_bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
