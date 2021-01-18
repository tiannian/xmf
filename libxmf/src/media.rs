use crate::{Block, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub enum AlignType {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "center")]
    Center,
}

pub struct Image {
    pub url: String,
    pub width: usize,
    pub height: usize,
    pub align: AlignType,
}

impl Block for Image {
    fn ty() -> String
    where
        Self: Sized,
    {
        "image".into()
    }

    fn to_json(&self, json: &mut Json) {
        let d = json!({
            "type": Self::ty(),
            "url": self.url,
            "width": self.width,
            "height": self.height,
            "align": serde_json::to_value(&self.align).unwrap(),
        });

        json.insert_single(d);
    }
}

pub struct Video {
    pub url: String,
    pub play: bool,
    pub seek: usize,
    pub width: usize,
    pub height: usize,
    pub align: AlignType,
    pub volume: u16,
}

impl Block for Video {
    fn ty() -> String
    where
        Self: Sized,
    {
        "video".into()
    }

    fn to_json(&self, json: &mut Json) {
        let d = json!({
            "type": Self::ty(),
            "url": self.url,
            "play": self.play,
            "seek": self.seek,
            "width": self.width,
            "height": self.height,
            "align": serde_json::to_value(&self.align).unwrap(),
            "volume": self.volume,
        });

        json.insert_single(d);
    }
}

pub struct Audio {
    pub url: String,
    pub play: bool,
    pub seek: usize,
    pub volume: u16,
}

impl Block for Audio {
    fn ty() -> String
    where
        Self: Sized,
    {
        "audio".into()
    }

    fn to_json(&self, json: &mut Json) {
        let d = json!({
            "type": Self::ty(),
            "url": self.url,
            "play": self.play,
            "seek": self.seek,
            "volume": self.volume,
        });

        json.insert_single(d);
    }
}
