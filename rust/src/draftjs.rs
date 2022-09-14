use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentState {
    #[serde(rename = "entityMap")]
    pub entity_map: EntityMap,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityMap(HashMap<usize, Entity>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    #[serde(rename = "type")]
    pub entity_type: String,
    pub mutability: String,
    pub data: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub key: String,
    pub text: String,
    #[serde(rename = "type")]
    pub block_type: String,
    pub depth: usize,
    #[serde(rename = "inlineStyleRanges")]
    pub inline_style_ranges: Vec<String>,
    #[serde(rename = "entityRanges")]
    pub entity_ranges: Vec<EntityRange>,
    pub data: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityRange {
    pub offset: usize,
    pub length: usize,
    pub key: usize,
}
