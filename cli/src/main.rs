use horizon_lib::StructuredAttrs;
use serde_json;

fn main() {
    let struct_attrs: StructuredAttrs = StructuredAttrs::from_cwd();

    // println!("{:#?}", serde_json::to_string(&struct_attrs));

    let output_file: OutputFile = OutputFile::from_env();
}
