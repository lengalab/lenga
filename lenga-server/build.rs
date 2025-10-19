fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &[
            "lenga.proto",
            // C Lenga Definitions
            "c/lenga.proto",
            // Other definitions
        ],
        &["proto"],
    )?;
    Ok(())
}
