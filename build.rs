extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-search=/opt/conda/lib");
    println!("cargo:rustc-link-lib=protobuf");
    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-lib=taso_runtime");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        .enable_cxx_namespaces()
        .opaque_type("taso::Op")
        .opaque_type("taso::Tensor")
        .opaque_type("taso::Model")
        .whitelist_type("taso::Edge")
        .whitelist_type("taso::Graph")
        .blacklist_function("taso::Graph.export_op")
        .whitelist_function("taso::Graph.Graph")
        .opaque_type("std::*")
        .opaque_type("std::basic_ostream")
        .opaque_type("std::basic_ofstream")
        .opaque_type("std::__basic_file")
        .opaque_type("std::basic_string")
        .opaque_type("std::map")
        .opaque_type("std::vector")
        .opaque_type("std::basic_istream_sentry_traits_type")
        .opaque_type("std::basic_istream_sentry___streambuf_type")
        .opaque_type("std::basic_istream_sentry___istream_type")

        .whitelist_function("taso::Graph.total_cost")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
