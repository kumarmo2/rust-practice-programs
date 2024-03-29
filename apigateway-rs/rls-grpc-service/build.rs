fn main() -> std::io::Result<()> {
    // NOTE: since the envoy related protos are not changed
    // these are built once for now.
    println!("in rls build file");
    tonic_build::configure()
        .build_server(true)
        .out_dir("../compiled_protos")
        .compile(
            &[
                "../protos/envoy/service/ratelimit/v3/rls.proto",
                "../protos/grpc/health/v1/health.proto",
            ],
            &["../protos/"],
        )
        .unwrap();
    Ok(())
}
