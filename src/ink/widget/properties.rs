use serde::{Deserialize, Serialize};

use crate::{HandleId, Name};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type", rename_all = "camelCase")]
pub struct inkPropertyBinding {
    pub property_name: Name,
    pub style_path: Name,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub struct inkPropertyManager {
    pub bindings: Vec<inkPropertyBinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PropertyManager {
    pub handle_id: HandleId,
    pub data: inkPropertyManager,
}
