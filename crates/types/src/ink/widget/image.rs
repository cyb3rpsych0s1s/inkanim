use inkanim_macros::Reds;
use serde::{Deserialize, Serialize};

use crate::DepotPath;

use super::Flags;

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct inkTextureAtlas {
    depot_path: DepotPath,
    flags: Flags,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Reds)]
pub enum inkBrushMirrorType {
    NoMirror = 0,
    Horizontal = 1,
    Vertical = 2,
    Both = 3,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Reds)]
pub enum inkBrushTileType {
    NoTile = 0,
    Horizontal = 1,
    Vertical = 2,
    Both = 3,
}
