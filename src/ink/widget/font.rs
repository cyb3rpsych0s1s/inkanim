use serde::{Deserialize, Serialize};

use crate::Name;

use super::Flags;

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct inkFontFamilyResource {
    depot_path: Name,
    flags: Flags,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct fontStyle(Name);

#[allow(non_camel_case_types, clippy::enum_variant_names)]
#[derive(Debug, Serialize, Clone, Deserialize)]
pub enum textLetterCase {
    OriginalCase = 0,
    UpperCase = 1,
    LowerCase = 2,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Clone, Deserialize)]
pub enum textHorizontalAlignment {
    Left = 0,
    Center = 1,
    Right = 2,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Clone, Deserialize)]
pub enum textVerticalAlignment {
    Top = 0,
    Center = 1,
    Bottom = 2,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Clone, Deserialize)]
pub enum textOverflowPolicy {
    None = 0,
    DotsEnd = 1,
    DotsEndLastLine = 2,
    AutoScroll = 3,
    PingPongScroll = 4,
    AdjustToSize = 5,
}
