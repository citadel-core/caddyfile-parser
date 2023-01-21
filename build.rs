use std::env;
use std::path::PathBuf;

fn main() {
    gobuild::Build::new()
        .file("main.go")
        .compile("caddyfile-to-json");

    let bindings = bindgen::Builder::default()
        .header(std::env::var("OUT_DIR").unwrap() + "/libcaddyfile-to-json.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
