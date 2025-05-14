use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Error, Write};
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
    Neim: String,
    spinyrz: ClusterMethods,
}

#[derive(Serialize, Deserialize)]
pub struct Node {
    Neim: String,
    spinyrz: NodeMethods,
}
