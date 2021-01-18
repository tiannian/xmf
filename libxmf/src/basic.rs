use crate::{Block, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct Text {
    pub data: String,
}

impl Block for Text {
    fn ty() -> String
    where
        Self: Sized,
    {
        "text".into()
    }

    fn to_json(&self, json: &mut Json) {
        let d = json!({
            "type": "text",
            "data": self.data
        });

        json.insert_single(d);
    }
}

pub struct Heading {
    pub level: u8,
    pub data: String,
}

impl Block for Heading {
    fn ty() -> String
    where
        Self: Sized,
    {
        "heading".into()
    }

    fn to_json(&self, json: &mut Json) {
        let d = json!({
            "type": "heading",
            "data": self.data,
            "level": self.level,
        });

        json.insert_single(d);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ListType {
    #[serde(rename = "bulleted")]
    Bulleted,
    #[serde(rename = "numbered")]
    Numbered,
}

pub struct ListItem {
    pub is_selectable: bool,
    pub data: Box<dyn Block>,
}

pub struct List {
    pub ty: ListType,
    pub data: Vec<ListItem>,
}

impl Block for List {
    fn ty() -> String
    where
        Self: Sized,
    {
        "list".into()
    }

    fn to_json(&self, json: &mut Json) {
        let mut v = Vec::new();
        for item in &self.data {
            (*item.data).to_json(json);
            let d = json!({
                "is_selectable": item.is_selectable,
                "block_index": json.blocks.len() - 1,
            });
            v.push(d);
        }
        let d = json!({
            "type": "list",
            "list": serde_json::to_value(&self.ty).unwrap(),
            "data": v
        });
        json.blocks.push(d);
    }
}

pub struct Qoute {
    pub data: Box<dyn Block>,
}

impl Block for Qoute {
    fn ty() -> String
    where
        Self: Sized,
    {
        "qoute".into()
    }

    fn to_json(&self, json: &mut Json) {
        (*self.data).to_json(json);
        let d = json!({
            "type": "qoute",
            "block_index": json.blocks.len() - 1,
        });

        json.insert_single(d);
    }
}

pub struct Divider {}

impl Block for Divider {
    fn ty() -> String
    where
        Self: Sized,
    {
        "divider".into()
    }

    fn to_json(&self, json: &mut Json) {
        let d = json!({
            "type": Self::ty(),
        });

        json.insert_single(d);
    }
}

pub struct Link {
    pub data: String,
}

impl Block for Link {
    fn ty() -> String
    where
        Self: Sized,
    {
        "link".into()
    }

    fn to_json(&self, json: &mut Json) {
        let d = json!({
            "type": Self::ty(),
            "data": self.data
        });

        json.insert_single(d);
    }
}

#[cfg(test)]
mod tests {
    use super::Block;
    use super::Heading;
    use super::Json;
    use super::List;
    use super::ListItem;
    use super::ListType;
    use super::Qoute;
    use super::Text;

    #[test]
    fn test_list() {
        let mut json = Json::default();
        let text = Text {
            data: "text".into(),
        };
        let heading = Heading {
            data: "etehe".into(),
            level: 2,
        };
        let mut list = List {
            ty: ListType::Numbered,
            data: Vec::new(),
        };

        list.data.push(ListItem {
            data: Box::new(text),
            is_selectable: true,
        });

        list.to_json(&mut json);

        let qoute = Qoute {
            data: Box::new(heading),
        };

        qoute.to_json(&mut json);

        json.debug_json();
    }
}
