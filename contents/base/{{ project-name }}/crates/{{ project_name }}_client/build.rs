use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rerun-if-changed=../../specs/self");

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("{{ project_name }}_descriptor.bin"))
        .build_server(false)
        .build_client(true)
        .compile_protos(&["../../specs/self/{{ project_name }}.proto"], &["../../specs/self"])?;

    Ok(())
}
