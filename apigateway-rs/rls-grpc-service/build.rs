fn main() -> std::io::Result<()> {
    tonic_build::configure()
        .build_server(true)
        .out_dir("../compiled_protos")
        .compile(
            &["./protos/envoy/service/ratelimit/v3/rls.proto"],
            &["./protos/"],
        )?;
    Ok(())
}
