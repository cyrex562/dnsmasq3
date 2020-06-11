fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .format(true)
        .out_dir("src")
        .compile(
            &[
                "src/pt_common.proto",
                "src/pt_host_agent.proto",
                "src/pt_controller.proto",
            ],
            &["src"],
        )?;
    Ok(())
}
