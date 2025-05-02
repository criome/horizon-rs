use horizon_lib::StructuredAttrs;
use serde_json;

fn main() {
    let struct_attrs = StructuredAttrs::from_cwd();
    println!("{:#?}", serde_json::to_string(&struct_attrs));
}
