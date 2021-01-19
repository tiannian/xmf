use crate::{Block, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct Snippet {
    pub lang: String,
    pub data: String,
}

impl Block for Snippet {
    fn ty() -> String
    where
        Self: Sized,
    {
        "snippet".into()
    }

    fn to_json(&self, json: &mut Json) {
        let d = json!({
            "type": Self::ty(),
            "lang": self.lang,
            "code": self.code,
        });

        json.insert_single(d);
    }
}


