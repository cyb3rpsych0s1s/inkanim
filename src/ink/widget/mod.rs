mod implementation;

pub use implementation::ByIndex;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{HandleId, InkWrapper};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Flags {
    Soft,
}

/// see [NativeDB](https://nativedb.red4ext.com/inkCanvasWidget)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub struct inkCanvasWidget {
    pub children: InkWrapper<inkMultiChildren>,
    pub name: String,
}

/// see [NativeDB](https://nativedb.red4ext.com/inkHorizontalPanelWidget)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub struct inkHorizontalPanelWidget {
    pub children: InkWrapper<inkMultiChildren>,
    pub name: String,
}

/// see [NativeDB](https://nativedb.red4ext.com/inkVerticalPanelWidget)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub struct inkVerticalPanelWidget {
    pub children: InkWrapper<inkMultiChildren>,
    pub name: String,
}

/// see [NativeDB](https://nativedb.red4ext.com/inkMultiChildren)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub struct inkMultiChildren {
    pub children: Vec<InkWrapper<Widget>>,
}

/// see [NativeDB](https://nativedb.red4ext.com/inkTextWidget)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub struct inkTextWidget {
    pub name: String,
}

/// any widget
#[allow(clippy::enum_variant_names)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Widget {
    inkCanvasWidget(inkCanvasWidget),
    inkMultiChildren(inkMultiChildren),
    inkTextWidget(inkTextWidget),
    inkHorizontalPanelWidget(inkHorizontalPanelWidget),
    inkVerticalPanelWidget(inkVerticalPanelWidget),
}

/// see [NativeDB](https://nativedb.red4ext.com/inkWidgetLibraryItemInstance)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
#[serde(rename_all = "camelCase")]
pub struct inkWidgetLibraryItemInstance {
    pub root_widget: InkWrapper<inkCanvasWidget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub version: usize,
    pub build_version: usize,
    pub root_chunk: inkWidgetLibraryItemInstance,
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
pub struct File {
    pub header: Header,
    pub data: Data,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Package {
    pub file: File,
}

/// see [NativeDB](https://nativedb.red4ext.com/inkWidgetLibraryItem)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub struct inkWidgetLibraryItem {
    pub name: String,
    pub package: Package,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnimationLibraryResRef {
    depot_path: PathBuf,
    flags: Flags,
}

/// see [NativeDB](https://nativedb.red4ext.com/inkWidgetLibraryResource)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
#[serde(rename_all = "camelCase")]
pub struct inkWidgetLibraryResource {
    pub animation_library_res_ref: AnimationLibraryResRef,
    pub external_dependencies_for_internal_items: Vec<AnimationLibraryResRef>,
    pub library_items: Vec<inkWidgetLibraryItem>,
}

impl inkWidgetLibraryItemInstance {
    pub fn get_widget_kind(&self, path: &[usize]) -> Option<String> {
        let mut parent: Option<Widget> = Some(Widget::inkMultiChildren(
            self.root_widget.data.children.data.clone(),
        ));
        let last = path.len() - 1;
        for (i, idx) in path.iter().enumerate() {
            if parent.is_none() {
                break;
            }
            if let Some(ref child) = parent.as_ref().unwrap().by_index(*idx) {
                match child {
                    Widget::inkCanvasWidget(node) => {
                        if i == last {
                            return Some("inkCanvasWidget".to_string());
                        }
                        parent = Some(Widget::inkCanvasWidget(node.clone()));
                        continue;
                    }
                    Widget::inkHorizontalPanelWidget(node) => {
                        if i == last {
                            return Some("inkHorizontalPanelWidget".to_string());
                        }
                        parent = Some(Widget::inkHorizontalPanelWidget(node.clone()));
                        continue;
                    }
                    Widget::inkVerticalPanelWidget(node) => {
                        if i == last {
                            return Some("inkVerticalPanelWidget".to_string());
                        }
                        parent = Some(Widget::inkVerticalPanelWidget(node.clone()));
                        continue;
                    }
                    Widget::inkMultiChildren(node) => {
                        panic!("encountered unexpected inkMultiChildren at index {idx}");
                    }
                    Widget::inkTextWidget(leaf) => return Some("inkTextWidget".to_string()),
                }
            }
        }
        None
    }
    pub fn get_path_names(&self, path: &[usize]) -> Option<Vec<String>> {
        let mut names: Vec<String> = vec![];
        let mut parent: Option<Widget> = Some(Widget::inkMultiChildren(
            self.root_widget.data.children.data.clone(),
        ));

        for idx in path {
            if parent.is_none() {
                break;
            }
            if let Some(ref child) = parent.unwrap().by_index(*idx) {
                match child {
                    Widget::inkCanvasWidget(node) => {
                        names.push(node.name.clone());
                        parent = Some(Widget::inkCanvasWidget(node.clone()));
                        continue;
                    }
                    Widget::inkMultiChildren(_)
                    | Widget::inkHorizontalPanelWidget(_)
                    | Widget::inkVerticalPanelWidget(_) => {
                        panic!("encountered unexpected inkMultiChildren at index {idx}");
                    }
                    Widget::inkTextWidget(leaf) => {
                        names.push(leaf.name.clone());
                        break;
                    }
                }
            }
            return None;
        }
        Some(names)
    }
}

/// widget aggregated informations summary
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetSummary {
    /// unique handle ID
    pub HandleId: HandleId,
    /// widget name
    pub Name: String,
    // pub Path: Vec<usize>,
}
