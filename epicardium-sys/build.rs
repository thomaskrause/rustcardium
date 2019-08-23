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

    let firmware_build_dir = env::var("EPICARDIUM_BUILD_DIR").expect("You need to set the EPICARDIUM_BUILD_DIR environment variable to the build directory of the firmware");

    println!("cargo:rustc-link-search={}/epicardium", firmware_build_dir);
    println!("cargo:rustc-link-search={}/l0dables/lib/", firmware_build_dir);
    println!(
        "cargo:rustc-link-search={}/lib/sdk/Libraries/MAX32665PeriphDriver",
        firmware_build_dir
    );

    println!("cargo:rustc-link-lib=static=PeriphDriver");
    println!("cargo:rustc-link-lib=static=api-caller");
    println!("cargo:rustc-link-lib=static=l0dable-startup");
}
