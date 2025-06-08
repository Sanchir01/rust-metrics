use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let file_descriptor_path = out_dir.join("descriptor.bin");
    tonic_build::configure()
        .file_descriptor_set_path(file_descriptor_path)
        .build_server(true)
        .out_dir("./")
        .compile_protos(&["files/metrics.proto"], &["./"])?;

    tonic_build::compile_protos("files/metrics.proto")?;
    Ok(())
}