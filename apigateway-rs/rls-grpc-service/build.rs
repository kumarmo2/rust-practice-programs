fn main() -> std::io::Result<()> {
    // NOTE: since the envoy related protos are not changed
    // these are built once for now.
    // tonic_build::configure()
    // .build_server(true)
    // .out_dir("../compiled_protos")
    // .compile(
    // &["./protos/envoy/service/ratelimit/v3/rls.proto"],
    // &["./protos/"],
    // )?;
    Ok(())
}
