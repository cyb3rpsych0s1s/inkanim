use inkanim_macros::Reds;
use serde::{Deserialize, Serialize};

use crate::DepotPath;

use super::Flags;

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Serialize, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct inkTextureAtlas {
    depot_path: DepotPath,
    flags: Flags,
}

#[allow(non_camel_case_types)]
#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Reds,
)]
pub enum inkBrushMirrorType {
    #[default]
    NoMirror = 0,
    Horizontal = 1,
    Vertical = 2,
    Both = 3,
}

#[allow(non_camel_case_types)]
#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Reds,
)]
pub enum inkBrushTileType {
    #[default]
    NoTile = 0,
    Horizontal = 1,
    Vertical = 2,
    Both = 3,
}
