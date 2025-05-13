// TODO: Move some of this to a Nix library crate

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct StructuredAttrs {
    name: String,
    builder: Box<Path>,
    system: System,
    outputs: Outputs,

    #[serde(default)]
    args: Args,

    #[serde(flatten)]
    attrs: HashMap<String, Value>,
}

#[derive(Default, Serialize, Deserialize)]
struct Outputs(HashMap<String, Output>);

#[derive(Default, Serialize, Deserialize)]
struct Output {
    path: Option<String>,
    method: Option<String>,
    hash_algo: Option<String>,
    hash: Option<String>,
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

#[derive(Debug)]
pub struct OutputFile(File);

impl StructuredAttrs {
    pub fn from_cwd() -> Self {
        let attrs_json_file: &Path = Path::new("./.attrs.json");
        let data: String = fs::read_to_string(attrs_json_file).expect("Unable to read file");
        let value: Self = serde_json::from_str(&data).expect("JSON does not have correct format.");
        return value;
    }
}

impl Default for OutputFile {
    fn default() -> Self {
        let nix_out_str = std::env::var("out").expect("Missing $out environment");
        // TODO
        let output_dir_str = nix_out_str + "/etc";
        let out_path_str = output_dir_str + "/horizon.json";
        let file: File = File::create(Path::new(&out_path_str)).expect("Error");
        OutputFile(file)
    }
}

impl TryFrom<StructuredAttrs> for OutputFile {
    type Error = &'static str;

    fn try_from(value: StructuredAttrs) -> Result<Self, Self::Error> {
        let out_path_str = (value
            .outputs
            .0
            .get("out")
            .expect("Error: missing `out` output"))
        .path
        .clone()
        .expect("");
        let file: File = File::create(Path::new(&out_path_str)).expect("Error");
        Ok(OutputFile(file))
    }
}

impl Write for OutputFile {
    fn write(&mut self, data: &[u8]) -> Result<usize, Error> {
        self.0.write(&data)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.0.flush()
    }
}
