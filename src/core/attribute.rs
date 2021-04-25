use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub base: i32,
    pub drain: i32,
    pub damage: i32,
    pub modifier: i32,
}

impl Attribute {
    pub fn current(&self) -> i32 {
        self.base - self.damage - self.drain + self.modifier
    }

    pub fn max(&self) -> i32 {
        self.base - self.drain + self.modifier
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum AttributeKind {
    Hp,
    Mp,
    Attack,
    Defence,
    Sanity,
}
