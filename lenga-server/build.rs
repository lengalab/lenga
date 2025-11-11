fn main() -> Result<(), Box<dyn std::error::Error>> {
    let regen = std::env::var("REGEN_PROTOS").is_ok();
    if !regen {
        // No-op for normal builds (including crates.io users).
        return Ok(());
    }

    tonic_build::configure()
        .out_dir("./lenga-server/rpc/generated")
        .compile(
            &[
                "lenga.proto",
                // C Lenga Definitions
                "c/lenga.proto",
                // Other definitions
            ],
            &["./lenga-server/rpc/protos"],
        )?;
    Ok(())
}
