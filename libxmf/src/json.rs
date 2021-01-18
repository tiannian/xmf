use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRelation {
    pub b: usize,
    pub e: usize,
}

impl JsonRelation {
    pub fn new(b: usize, e: usize) -> Self {
        JsonRelation { b, e }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Json {
    pub blocks: Vec<Value>,
}

impl Default for Json {
    fn default() -> Self {
        Json { blocks: Vec::new() }
    }
}

impl Json {
    pub fn insert_single(&mut self, v: Value) {
        self.blocks.push(v);
    }

    pub fn debug_json(&self) {
        println!("{}", serde_json::to_string_pretty(self).unwrap());
    }
}
