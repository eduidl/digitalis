use std::{fs, path::Path};

use protobuf_codegen::Codegen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must be inside workspace");

    let schemas_proto_dir = workspace_dir.join("3rdparty/schemas/schemas/proto");
    let foxglove_dir = schemas_proto_dir.join("foxglove");

    let mut protos = vec![];
    for entry in fs::read_dir(&foxglove_dir)? {
        let path = entry?.path();
        if path.is_file() && path.extension() == Some("proto".as_ref()) {
            protos.push(path);
        }
    }

    let out_dir = workspace_dir.join("protobuf/src/msgs");

    Codegen::new()
        .out_dir(&out_dir)
        .inputs(&protos)
        .include(&schemas_proto_dir)
        .run()?;

    Ok(())
}
