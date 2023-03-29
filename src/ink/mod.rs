use serde::{Deserialize, Deserializer, Serialize};
use serde_aux::prelude::*;

use self::{
    anim::{InkAnimSequence, Target},
    widget::SiblingOrNested,
};

/// everything related to *.inkanim*
pub mod anim;
/// everything related to *.inkwidget*
pub mod widget;

/// deserialize handle ID (from number or string)
pub fn deserialize_handle_id_from_string<'de, D>(deserializer: D) -> Result<HandleId, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(HandleId(deserialize_number_from_string(deserializer)?))
}

/// see [NativeDB](https://nativedb.red4ext.com/Vector2)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(tag = "$type")]
#[serde(rename_all = "PascalCase")]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

/// see [NativeDB](https://nativedb.red4ext.com/HDRColor)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(tag = "$type")]
#[serde(rename_all = "PascalCase")]
pub struct HDRColor {
    pub alpha: f32,
    pub blue: f32,
    pub green: f32,
    pub red: f32,
}

/// asset handle ID
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HandleId(u32);

/// wrapper with handle ID
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InkWrapper<T> {
    #[serde(deserialize_with = "deserialize_handle_id_from_string")]
    pub handle_id: HandleId,
    pub data: T,
}

/// specific resource ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CName(String);

impl<T> std::fmt::Display for InkWrapper<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) {}", self.handle_id, self.data)
    }
}

impl std::fmt::Display for HandleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "🔑 {}", self.0)
    }
}

/// animation aggregated informations summary
#[allow(dead_code, non_snake_case)]
#[derive(Debug)]
pub struct PathSummary {
    /// animation name
    Name: String,
    /// unique handle ID
    HandleId: HandleId,
    /// index in sequence
    Index: usize,
    /// path to the nested element
    Path: Vec<usize>,
}

impl InkAnimSequence {
    /// summarize all paths matching sequences of digits
    pub fn get_path_indexes_matching(&self, searched: &[usize]) -> Vec<PathSummary> {
        let count = searched.len();
        let _last = count - 1;
        let mut out = vec![];
        for (target_index, target) in self.targets.iter().enumerate() {
            match target {
                Target::WithHandleId(ref handle) => {
                    let path = &handle.data.path;
                    if path.sibling_or_nested(searched) {
                        out.push(PathSummary {
                            Name: self.name.clone(),
                            HandleId: handle.handle_id,
                            Index: target_index,
                            Path: path.clone(),
                        });
                    }
                }
                _ => continue,
            }
        }
        out
    }
}
