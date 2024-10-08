use std::{
    env,
    fs::copy,
    path::{Path, PathBuf},
};

fn main() {
    if !Path::new("src/message_types/handwriting/handwriting_proto.rs").exists() {
        protobuf_codegen::Codegen::new()
            .pure()
            .input("src/message_types/handwriting/handwriting.proto")
            .include(".")
            .cargo_out_dir("p")
            .run_from_script();

        // Move generated file to correct location
        let mut generated = PathBuf::from(env::var("OUT_DIR").unwrap());
        generated.push("p");
        generated.push("handwriting.rs");
        copy(
            generated,
            "src/message_types/handwriting/handwriting_proto.rs",
        )
        .unwrap();
    }
}
