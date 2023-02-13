pub(crate) mod implementation;

pub use implementation::*;

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
