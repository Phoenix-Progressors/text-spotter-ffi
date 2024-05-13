// extern crate bindgen;
// extern crate cc;
use std::{env, path::PathBuf};

fn main() {
    // Specify the path to the textspot ter library file
    let lib_path = "./textspotter/textspotter/lib";
    // Link the textspotter library
    // println!("cargo:rustc-link-lib=opencv4");
    println!("cargo:rustc-link-lib=/opt/homebrew/Cellar/opencv/4.9.0_8.reinstall/lib/pkgconfig/");
    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:rustc-link-lib=static=textspotter");

    // Specify the path to the textspotter library header file
    let include_path = "./textspotter/textspotter/include";

    // Compile the textspotter library
    cc::Build::new()
        .file("./textspotter/textspotter/src/textspotter.cpp")
        .include(include_path)
        .compile("textspotter");

    // Generate bindings using bindgen
    let bindings = bindgen::Builder::default()
        .header("./textspotter/textspotter/include/textspotter/detect_read.hpp")
        .header("./textspotter/textspotter/include/textspotter/east_detector.hpp")
        .header("./textspotter/textspotter/include/textspotter/ocr.hpp")
        .header("./textspotter/textspotter/include/textspotter/result_type.hpp")
        .header("./textspotter/textspotter/include/textspotter/text_matching.hpp")
        .header("./textspotter/textspotter/include/textspotter/textspotter.hpp")
        .header("./textspotter/textspotter/include/textspotter/utility.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to a file
    // let out_path = std::path::PathBuf::from("./src");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
