use serde::{Deserialize, Serialize};

use super::Position;
#[derive(Debug, Serialize, Deserialize)]
pub struct Teleportation {
    pub position: Position,
    pub teleport: Position,
}
