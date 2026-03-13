fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    let protoc_include = protoc_bin_vendored::include_path()?;

    std::env::set_var("PROTOC", protoc);
    std::env::set_var("PROTOC_INCLUDE", &protoc_include);

    tonic_prost_build::configure().compile_protos(
        &[
            "../../proto/auth.proto",
            "../../proto/common.proto",
        ],
        &[
            "../../proto",
            protoc_include.to_str().unwrap(),
        ],
    )?;

    Ok(())
}
