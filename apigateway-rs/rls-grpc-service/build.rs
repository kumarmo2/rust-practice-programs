fn main() -> std::io::Result<()> {
    tonic_build::configure()
        .build_server(true)
        .out_dir("../compiled_protos")
        .compile(&["./envoy/service/ratelimit/v3/rls.proto"], &["."])?;
    Ok(())
}
