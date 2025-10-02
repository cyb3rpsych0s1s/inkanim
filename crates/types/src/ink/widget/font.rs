use inkanim_macros::Reds;
use serde::{Deserialize, Serialize};

use crate::{DepotPath, Name};

use super::Flags;

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Serialize, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct inkFontFamilyResource {
    depot_path: DepotPath,
    flags: Flags,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Serialize, Clone, Deserialize, PartialEq)]
pub struct fontStyle(Name);

#[allow(non_camel_case_types, clippy::enum_variant_names)]
#[derive(Debug, Default, Serialize, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Reds)]
pub enum textLetterCase {
    #[default]
    OriginalCase = 0,
    UpperCase = 1,
    LowerCase = 2,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Serialize, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Reds)]
pub enum textHorizontalAlignment {
    #[default]
    Left = 0,
    Center = 1,
    Right = 2,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Serialize, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Reds)]
pub enum textVerticalAlignment {
    Top = 0,
    #[default]
    Center = 1,
    Bottom = 2,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Serialize, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Reds)]
pub enum textOverflowPolicy {
    #[default]
    None = 0,
    DotsEnd = 1,
    DotsEndLastLine = 2,
    AutoScroll = 3,
    PingPongScroll = 4,
    AdjustToSize = 5,
}
