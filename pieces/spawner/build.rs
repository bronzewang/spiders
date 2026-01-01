// use std::{env, path::PathBuf};
// use prost_wkt_build::*;

// fn main() {
//     let out = PathBuf::from(env::var("OUT_DIR").unwrap());
//     let descriptor_file = out.join("descriptors.bin");
//     let mut prost_build = prost_build::Config::new();
//     prost_build
//         .type_attribute(
//             ".",
//             "#[derive(serde::Serialize,serde::Deserialize)]"
//         )
//         .file_descriptor_set_path(&descriptor_file)
//         .compile_protos(
//             &[
//                 "utils/whisper.proto"
//             ],
//             &["utils/"],
//         )
//         .unwrap();

//     let descriptor_bytes =
//         std::fs::read(descriptor_file)
//         .unwrap();

//     let descriptor =
//         FileDescriptorSet::decode(&descriptor_bytes[..])
//         .unwrap();

//     prost_wkt_build::add_serde(out, descriptor);
// }

fn main() -> std::io::Result<()> {
    tonic_build::configure()
        .type_attribute(
            ".",
            "#[derive(serde::Serialize,serde::Deserialize)]"
        )
        .compile(&["utils/greeter.proto"], &["utils"])
        .unwrap();

    Ok(())
}

