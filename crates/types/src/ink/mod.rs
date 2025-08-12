use std::path::PathBuf;

use serde::{Deserialize, Serialize, ser::SerializeStruct};
use serde_aux::prelude::*;

use self::{
    anim::{InkAnimSequence, Target},
    widget::SiblingOrNested,
};
mod conversion;
use conversion::deserialize_lockey_from_anything;

/// everything related to *.inkanim*
pub mod anim;
/// everything related to *.inkwidget*
pub mod widget;

#[derive(Debug, Default, Clone, Deserialize, PartialEq)]
pub struct Name {
    #[serde(rename = "$type")]
    r#type: String,
    #[serde(rename = "$storage")]
    storage: String,
    #[serde(rename = "$value")]
    value: String,
}

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.value)
    }
}

impl Name {
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Storage {
    #[default]
    String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourcePath {
    #[serde(rename = "$storage")]
    storage: Storage,
    #[serde(rename = "$value")]
    value: PathBuf,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(tag = "$type")]
pub enum DepotPath {
    ResourcePath(ResourcePath),
}

impl Serialize for DepotPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::ResourcePath(x) => {
                serializer.serialize_str(x.value.as_path().as_os_str().to_str().unwrap_or_default())
            }
        }
    }
}

impl Default for DepotPath {
    fn default() -> Self {
        Self::ResourcePath(ResourcePath::default())
    }
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
#[derive(Debug, Default, Clone, Deserialize, PartialEq, PartialOrd)]
#[serde(tag = "$type")]
#[serde(rename_all = "PascalCase")]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Serialize for Vector2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Vector2", 2)?;
        s.serialize_field("X", &self.x)?;
        s.serialize_field("Y", &self.y)?;
        s.end()
    }
}

/// see [NativeDB](https://nativedb.red4ext.com/HDRColor)
#[derive(Debug, Default, Clone, Deserialize, PartialEq, PartialOrd)]
#[serde(tag = "$type")]
#[serde(rename_all = "PascalCase")]
pub struct HDRColor {
    pub alpha: f32,
    pub blue: f32,
    pub green: f32,
    pub red: f32,
}

impl Serialize for HDRColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("HDRColor", 4)?;
        s.serialize_field("alpha", &self.alpha)?;
        s.serialize_field("blue", &self.blue)?;
        s.serialize_field("green", &self.green)?;
        s.serialize_field("red", &self.red)?;
        s.end()
    }
}

/// asset handle ID
///
/// identifies the index in the graph.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct HandleId(#[serde(deserialize_with = "deserialize_number_from_string")] u32);

/// wrapper with handle ID
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InkWrapper<T> {
    pub handle_id: HandleId,
    pub data: T,
}

impl<T> Serialize for InkWrapper<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.data.serialize(serializer)
    }
}

impl<T> Default for InkWrapper<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            handle_id: Default::default(),
            data: Default::default(),
        }
    }
}

impl<T> PartialEq for InkWrapper<T>
where
    T: PartialEq<T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.handle_id == other.handle_id && self.data == other.data
    }
}

/// specific resource ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CName(String);

#[derive(Debug, Clone, Deserialize)]
pub enum LocKey {
    ID(u32),
    Value(String),
}

impl Default for LocKey {
    fn default() -> Self {
        LocKey::ID(0)
    }
}

impl Serialize for LocKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::ID(x) => serializer.serialize_u32(*x),
            Self::Value(x) => serializer.serialize_str(x.as_str()),
        }
    }
}

impl PartialEq for LocKey {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ID(lhs), Self::ID(rhs)) => lhs == rhs,
            (Self::Value(lhs), Self::Value(rhs)) => lhs == rhs,
            (Self::ID(lhs), Self::Value(rhs)) => &lhs.to_string() == rhs,
            (Self::Value(lhs), Self::ID(rhs)) => lhs == &rhs.to_string(),
        }
    }
}

/// specific translation ID
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct LocalizationString {
    #[serde(deserialize_with = "deserialize_lockey_from_anything")]
    value: Option<LocKey>,
}

impl Serialize for LocalizationString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self.value {
            Some(x)
                if *x != LocKey::default()
                    && *x != LocKey::Value("".into())
                    && *x != LocKey::ID(0) =>
            {
                serializer.serialize_some(x)
            }
            _ => serializer.serialize_none(),
        }
    }
}

pub fn is_any_default_localization_string(
    LocalizationString { value }: &LocalizationString,
) -> bool {
    match value {
        None => true,
        Some(LocKey::ID(0)) => true,
        Some(LocKey::Value(x)) if x.is_empty() => true,
        _ => false,
    }
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
