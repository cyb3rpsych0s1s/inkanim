use inkanim_macros::RedsValue;
use serde::{Deserialize, Serialize};

use crate::Name;

use super::Flags;

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct inkTextureAtlas {
    depot_path: Name,
    flags: Flags,
}

unsafe impl red4ext_rs::prelude::NativeRepr for inkTextureAtlas {
    const NAME: &'static str = "inkTextureAtlas";
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, RedsValue, PartialEq)]
pub enum inkBrushMirrorType {
    #[default]
    NoMirror = 0,
    Horizontal = 1,
    Vertical = 2,
    Both = 3,
}

unsafe impl red4ext_rs::prelude::NativeRepr for inkBrushMirrorType {
    const NAME: &'static str = "inkBrushMirrorType";
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, RedsValue, PartialEq)]
pub enum inkBrushTileType {
    #[default]
    NoTile = 0,
    Horizontal = 1,
    Vertical = 2,
    Both = 3,
}

unsafe impl red4ext_rs::prelude::NativeRepr for inkBrushTileType {
    const NAME: &'static str = "inkBrushTileType";
}
