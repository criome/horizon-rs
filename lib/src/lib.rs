use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub const HELLO: &str = "hello world";

#[derive(Serialize, Deserialize)]
pub struct StructuredAttrs {
    name: String,
    builder: Box<Path>,
    system: System,

    #[serde(default)]
    args: Args,

    #[serde(flatten)]
    attrs: HashMap<String, Value>,
}

#[derive(Default, Serialize, Deserialize)]
struct Args(Vec<String>);

#[derive(Serialize, Deserialize)]
pub enum System {
    #[serde(rename = "x86_64-linux")]
    X86_64Linux,
    #[serde(rename = "i686-linux")]
    I686Linux,
    #[serde(rename = "aarch64-linux")]
    Aarch64Linux,
    #[serde(rename = "armv7l-linux")]
    Armv7lLinux,
    #[serde(rename = "avr-linux")]
    AvrLinux,
}

impl StructuredAttrs {
    pub fn from_cwd() -> Self {
        let attrs_json_file: &Path = Path::new("./.attrs.json");
        let data: String = fs::read_to_string(attrs_json_file).expect("Unable to read file");
        let value: Self = serde_json::from_str(&data).expect("JSON does not have correct format.");
        return value;
    }
}
