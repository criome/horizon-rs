use crate::nix::{OutputFile, StructuredAttrs};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Horizon {
    #[serde(rename = "metastra")]
    cluster: Cluster,
    #[serde(rename = "astra")]
    node: Node,
    #[serde(rename = "exAstriz")]
    nodes: HashMap<Node>,
    users: Users,
}

#[derive(Serialize, Deserialize)]
pub struct Cluster {
    #[serde(rename = "neim")]
    name: String,
    #[serde(rename = "spinyrz")]
    methods: ClusterMethods,
}

#[derive(Serialize, Deserialize)]
pub struct Node {
    #[serde(rename = "neim")]
    name: String,
    #[serde(rename = "izFullyTrusted")]
    is_fully_trusted: Trust,
    #[serde(rename = "saizAtList")]
    is_size_at_least: Size,
    #[serde(rename = "izBildyr")]
    is_builder: String,
    #[serde(rename = "izDispatcyr")]
    is_dispatcher: String,
    #[serde(rename = "izNiksKac")]
    is_nix_cache: String,
    #[serde(rename = "izNiksCriodaizd")]
    has_nix_precriad: String,
    #[serde(rename = "izYggCriodaizd")]
    has_ygg_precriad: String,
    #[serde(rename = "izEseseitcCriodaizd")]
    has_ssh_precriad: String,
    #[serde(rename = "hazWireguardPreCriome")]
    has_wireguard_precriad: String,
    #[serde(rename = "izCriodaizd")]
    has_base_precriads: String,
    #[serde(rename = "eseseitcPreCriome")]
    ssh_precriom: String,
    #[serde(rename = "nixPreCriome")]
    nixPreCriome: String,
    #[serde(rename = "nixCacheDomain")]
    nixCacheDomain: String,
    #[serde(rename = "nixUrl")]
    nixUrl: String,
}

#[derive(Serialize, Deserialize)]
pub struct NodeMethods {
    izFullyTrusted: String,
    saizAtList: String,
    izBildyr: String,
    izDispatcyr: String,
    izNiksKac: String,
    izNiksCriodaizd: String,
    izYggCriodaizd: String,
    izNeksisCriodaizd: String,
    izEseseitcCriodaizd: String,
    hazWireguardPreCriome: String,
    izCriodaizd: String,
    eseseitcPreCriome: String,
    nixPreCriome: String,
    nixCacheDomain: String,
    nixUrl: String,
}

impl TryFrom<&StructuredAttrs> for Horizon {
    type Error = &'static str;

    fn try_from(value: StructuredAttrs) -> Result<Self, Self::Error> {
        let out_path_str = value
            .outputs
            .get("out")
            .expect("Error: missing `out` output");
        let file: File = File::create(Path::new(&out_path_str)).expect("Error");
        Ok(OutputFile(file))
    }
}
