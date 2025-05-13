use horizon_lib::nix::{OutputFile, StructuredAttrs};
use serde_json;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    let struct_attrs: StructuredAttrs = StructuredAttrs::from_cwd();
    let reserialized_data: String =
        serde_json::to_string(&struct_attrs).expect("Error Serializing Data");
    let mut output_file: OutputFile =
        OutputFile::try_from(struct_attrs).expect("Error: getting output file");
    output_file
        .write(reserialized_data.as_bytes())
        .expect("Error writing Data");
    Ok(())
}
