use serde::{Deserialize, Serialize};
use core::fmt::Debug;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

pub trait Span: Debug {
    fn ty() -> String
    where
        Self: Sized;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Editor {
    pub author: String,
    pub create_time: i64,
    pub update_time: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AlignType {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "center")]
    Center,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub align: AlignType,
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(skip)]
    pub spans: Vec<Box<dyn Span>>,
    pub editor: Editor,
}
