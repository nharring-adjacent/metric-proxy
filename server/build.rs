use std::path::PathBuf;
fn main() ->  Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
    .build_server(true)
    .build_client(true)
    .out_dir("src/metricproxy")
    .file_descriptor_set_path(PathBuf::from("src/metricproxy/metricproxy_descriptor.bin"))
    .compile(&["proto/metricproxy/proxy.proto"],&["proto"])?;
    Ok(())
}
