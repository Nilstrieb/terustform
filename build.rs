fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/tfplugin6.6.proto")?;
    Ok(())
}
