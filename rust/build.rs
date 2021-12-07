use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // Only run this script when either this file, or the DTO in dotnet changes
    // - This could also be a folder, which is recursively evaluated by the build
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../dotnet/SharedDtos.Contracts/MyDto.cs");

    // Execute build command
    let build_status = Command::new("dotnet")
        .args(["build", "../dotnet/SharedDtos.Contracts.Generator"])
        .status()
        .expect("failed to execute dotnet build process");
    assert!(build_status.success());

    // Create the 'schemas' dir if it doesn't exist
    let schema_dir = Path::new("./schemas");
    if !schema_dir.exists() {
        fs::create_dir_all(&schema_dir)
            .expect("failed to create base directory for generated schema files");
    }

    // Hard link the generated output JSON schemas
    for file in
        fs::read_dir("../dotnet/SharedDtos.Contracts.Generator/bin/Debug/net6.0/generated")
            .unwrap()
    {
        let src_file_path = file.unwrap().path();
        let dest_file_path = schema_dir.join(src_file_path.file_name().unwrap());

        if dest_file_path.exists() {
            fs::remove_file(&dest_file_path)
                .expect("failed to remove existing contract schema file");
        }

        fs::hard_link(&src_file_path, &dest_file_path)
            .expect("failed to link contract schema file into project");
    }
}
