extern crate cc;

fn main() {
    // Specify the path to the textspotter library header file
    let include_path = "./textspotter/textspotter/include";

    // Specify the path to the textspotter library file
    let lib_path = "./textspotter/textspotter/lib";

    // Compile the textspotter library
    cc::Build::new()
        .file("./textspotter/textspotter/src/textspotter.cpp")
        .include(include_path)
        .compile("textspotter");

    // Link the textspotter library
    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:rustc-link-lib=static=textspotter");
}
