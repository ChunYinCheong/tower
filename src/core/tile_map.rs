use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TileMap {
    pub id: i32,
    pub tiles: Vec<Vec<i32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tile {
    pub walkable: bool,
    pub kind: TileKind,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TileKind {
    /// TileSheet(id, x, y)
    TileSheet(i32, i32, i32),
    /// Image(file_path)
    Image(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileSheet {
    pub file_path: String,
    pub tile_width: i32,
    pub tile_height: i32,
}
