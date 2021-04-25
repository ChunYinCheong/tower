use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub struct Position {
    pub map_id: i32,
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn distance(a: &Position, b: &Position) -> i32 {
        if a.map_id == b.map_id {
            (a.x - b.x).abs() + (a.y - b.y).abs()
        } else {
            i32::MAX
        }
    }
}
