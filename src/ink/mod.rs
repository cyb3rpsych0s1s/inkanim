use std::path::PathBuf;

use inkanim_macros::RedsValue;
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name {
    #[serde(rename = "$type")]
    pub r#type: String,
    #[serde(rename = "$storage")]
    pub storage: String,
    #[serde(rename = "$value")]
    pub value: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct File<T> {
    pub header: Header,
    pub data: Data<T>,
}

/// see [NativeDB](https://nativedb.red4ext.com/Vector2)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, PartialOrd, RedsValue)]
#[serde(tag = "$type")]
#[serde(rename_all = "PascalCase")]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

unsafe impl red4ext_rs::prelude::NativeRepr for Vector2 {
    const NAME: &'static str = "Vector2";
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

unsafe impl red4ext_rs::prelude::NativeRepr for HDRColor {
    const NAME: &'static str = "HDRColor";
}

/// asset handle ID
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

unsafe impl red4ext_rs::prelude::NativeRepr for CName {
    const NAME: &'static str = "CName";
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LocKey {
    ID(u32),
    Value(String),
}

/// specific translation ID
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocalizationString {
    #[serde(deserialize_with = "deserialize_lockey_from_anything")]
    pub value: Option<LocKey>,
}

impl Default for LocalizationString {
    fn default() -> Self {
        Self { value: None }
    }
}

unsafe impl red4ext_rs::prelude::NativeRepr for LocalizationString {
    const NAME: &'static str = "LocalizationString";
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

#[cfg(test)]
mod tests {
    use inkanim_macros::RedsWidget;

    use crate::{widget::layout::inkEHorizontalAlign, RedsWidget, Vector2};

    // #[derive(RedsWidget, Debug, Clone, Default, PartialEq)]
    // pub struct TestParent {
    //     pub element: TestChild,
    // }
    // unsafe impl red4ext_rs::prelude::NativeRepr for TestParent {
    //     const NAME: &'static str = "TestParent";
    // }
    #[derive(RedsWidget, Debug, Clone, Default, PartialEq)]
    pub struct TestChild {
        pub content_h_align: inkEHorizontalAlign,
        pub size: Vector2,
    }
    unsafe impl red4ext_rs::prelude::NativeRepr for TestChild {
        const NAME: &'static str = "TChild";
    }
    #[test]
    fn reds_default() {
        let child = TestChild {
            content_h_align: inkEHorizontalAlign::Fill,
            size: Vector2 { x: 0., y: 0. },
        };
        assert_eq!(
            child.reds_widget("element", None),
            r#"let element = new TChild();"#
        );
    }
    #[test]
    fn reds_simple() {
        let child = TestChild {
            content_h_align: inkEHorizontalAlign::Center,
            size: Vector2 { x: 1., y: 0.6 },
        };
        assert_eq!(
            child.reds_widget("element", None),
            r#"let element = new TChild();
element.content_h_align = inkEHorizontalAlign.Center;
element.size = new Vector2(1., 0.6);"#
        );
    }
    #[test]
    fn reds_tree() {
        // let child = TestChild {
        //     content_h_align: inkEHorizontalAlign::Fill,
        //     size: Vector2 { x: 0., y: 0. },
        // };
        // let parent = TestParent { element: child };
        // assert_eq!(
        //     child.reds_widget("element", Some("parent")),
        //     r#"let element = new TChild();"#
        // );
    }
}
