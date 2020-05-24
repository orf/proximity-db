fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute(".", "#[derive(Serialize)]")
        .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
        .compile(&["proto/grpc.proto"], &["proto/"])?;
    Ok(())
}
