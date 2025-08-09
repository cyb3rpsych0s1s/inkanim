use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use self::{
    anim::{InkAnimSequence, Target},
    widget::SiblingOrNested,
};
mod conversion;
use conversion::deserialize_lockey_from_anything;

pub mod reds;

/// everything related to *.inkanim*
pub mod anim;
/// everything related to *.inkwidget*
pub mod widget;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name {
    #[serde(rename = "$type")]
    r#type: String,
    #[serde(rename = "$storage")]
    storage: String,
    #[serde(rename = "$value")]
    value: String,
}

impl Name {
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Storage {
    String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePath {
    #[serde(rename = "$storage")]
    storage: Storage,
    #[serde(rename = "$value")]
    value: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum DepotPath {
    ResourcePath(ResourcePath),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data<T> {
    pub version: usize,
    pub build_version: usize,
    pub root_chunk: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Header {
    wolven_kit_version: String,
    w_kit_json_version: String,
    game_version: usize,
    exported_date_time: chrono::DateTime<chrono::Utc>,
    data_type: String,
    archive_file_name: PathBuf,
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "🗃️  {}\n🐺 {} ({})",
            self.archive_file_name.display(),
            self.wolven_kit_version,
            self.w_kit_json_version,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct File<T> {
    pub header: Header,
    pub data: Data<T>,
}

impl<T> File<T> {
    pub fn resource(self) -> T {
        self.data.root_chunk
    }
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
///
/// identifies the index in the graph.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HandleId(#[serde(deserialize_with = "deserialize_number_from_string")] u32);

/// wrapper with handle ID
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InkWrapper<T> {
    pub handle_id: HandleId,
    pub data: T,
}

/// specific resource ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CName(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocKey {
    ID(u32),
    Value(String),
}

/// specific translation ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizationString {
    #[serde(deserialize_with = "deserialize_lockey_from_anything")]
    value: Option<LocKey>,
}

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
    Name: Name,
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
                Target::WithHandleId(handle) => {
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
