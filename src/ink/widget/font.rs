use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::CName;

use super::Flags;

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct inkFontFamilyResource {
    depot_path: PathBuf,
    flags: Flags,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct fontStyle(CName);
