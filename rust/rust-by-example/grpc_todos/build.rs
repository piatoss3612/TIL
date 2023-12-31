use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "proto/todos.proto"; // proto file path
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap()); // output directory
    tonic_build::configure() // configure the build
        .protoc_arg("--experimental_allow_proto3_optional") // enable proto3 optional feature
        .build_client(true) // build client
        .build_server(true) // also build server
        .file_descriptor_set_path(out_dir.join("todos_descriptor.bin")) // save descriptor set
        .out_dir("src") // save generated files to out_dir
        .compile(&[file_path], &["proto"])?; // compile proto file
    Ok(())
}
