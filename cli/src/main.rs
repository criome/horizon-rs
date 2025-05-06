use horizon_lib::{OutputFile, StructuredAttrs};
use serde_json;

fn main() {
    let struct_attrs: StructuredAttrs = StructuredAttrs::from_cwd();
    let reserialized_data: String = serde_json::to_string(&struct_attrs)?;
    let output_file: OutputFile = OutputFile::default();
    output_file.write(reserialized_data)
}
