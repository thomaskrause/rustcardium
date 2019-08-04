use std::env;
use std::path::PathBuf;

fn main() {


    println!("cargo:rerun-if-changed=epicardium-sys/epicardium.h");

    let bindings = bindgen::Builder::default()
        .header("epicardium.h")
        .use_core()
        .ctypes_prefix("cty")
        .generate()
        .expect("Unable to generate bindings to epicardium.h");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings to epicardium.h!");


    println!("cargo:rustc-link-search=ext/");
    println!("cargo:rustc-link-lib=static=PeriphDriver");
    println!("cargo:rustc-link-lib=static=api-caller");


}
