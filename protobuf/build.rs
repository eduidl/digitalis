use std::{env, fs};

use protobuf_codegen::Codegen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut protos = vec![];

    for entry in fs::read_dir("3rdparty/schemas/schemas/proto/foxglove")? {
        let path = entry?.path();
        if path.is_file() && path.extension() == Some("proto".as_ref()) {
            let path_str = path.to_str().unwrap();
            println!("cargo:rerun-if-changed={}", path_str);
            protos.push(path);
        }
    }

    let ret = Codegen::new()
        .out_dir("./src/msgs")
        .inputs(&protos)
        .include("3rdparty/schemas/schemas/proto")
        .run();

    if env::var("DOCS_RS").is_err() {
        ret?;
    }

    Ok(())
}
