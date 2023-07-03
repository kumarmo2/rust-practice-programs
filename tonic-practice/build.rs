fn main() -> std::io::Result<()> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        // make the output directory as src so that generated code
        // can be easily seen.
        .out_dir("./src")
        .compile(&["protos/hello-world.proto"], &["."])?;
    Ok(())
}
