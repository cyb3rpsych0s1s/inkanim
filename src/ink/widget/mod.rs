pub(crate) mod implementation;

use enum_dispatch::enum_dispatch;
pub use implementation::*;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{HandleId, InkWrapper};

/// belongs to the same level or is nested below, in a tree hierarchy
pub trait SiblingOrNested {
    fn sibling_or_nested(&self, searched: &[usize]) -> bool;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Flags {
    Soft,
}

macro_rules! native_compound_widget {
    ($ty:ident) => {
        /// see [NativeDB](https://nativedb.red4ext.com/$ty)
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $ty {
            pub children: InkWrapper<inkMultiChildren>,
            pub name: String,
        }
    };
}

macro_rules! native_leaf_widget {
    ($ty:ident) => {
        /// see [NativeDB](https://nativedb.red4ext.com/$ty)
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $ty {
            pub name: String,
        }
    };
}

native_compound_widget!(inkCanvasWidget);
native_compound_widget!(inkHorizontalPanelWidget);
native_compound_widget!(inkVerticalPanelWidget);
native_compound_widget!(inkScrollAreaWidget);
native_compound_widget!(inkUniformGridWidget);
native_compound_widget!(inkVirtualCompoundWidget);
native_compound_widget!(inkFlexWidget);
native_compound_widget!(inkCacheWidget);

/// see [NativeDB](https://nativedb.red4ext.com/inkMultiChildren)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct inkMultiChildren {
    pub children: Vec<InkWrapper<Widget>>,
}

native_leaf_widget!(inkTextWidget);
native_leaf_widget!(inkImageWidget);
native_leaf_widget!(inkVideoWidget);
native_leaf_widget!(inkMaskWidget);
native_leaf_widget!(inkBorderWidget);
native_leaf_widget!(inkShapeWidget);
native_leaf_widget!(inkCircleWidget);
native_leaf_widget!(inkRectangleWidget);
native_leaf_widget!(inkVectorGraphicWidget);

/// any widget
#[allow(clippy::enum_variant_names)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[enum_dispatch(Classname)]
#[serde(tag = "$type")]
pub enum Widget {
    inkMultiChildren(inkMultiChildren),

    inkCanvasWidget(inkCanvasWidget),
    inkHorizontalPanelWidget(inkHorizontalPanelWidget),
    inkVerticalPanelWidget(inkVerticalPanelWidget),
    inkScrollAreaWidget(inkScrollAreaWidget),
    inkUniformGridWidget(inkUniformGridWidget),
    inkVirtualCompoundWidget(inkVirtualCompoundWidget),
    inkFlexWidget(inkFlexWidget),
    inkCacheWidget(inkCacheWidget),

    inkTextWidget(inkTextWidget),
    inkImageWidget(inkImageWidget),
    inkVideoWidget(inkVideoWidget),
    inkMaskWidget(inkMaskWidget),
    inkBorderWidget(inkBorderWidget),
    inkShapeWidget(inkShapeWidget),
    inkCircleWidget(inkCircleWidget),
    inkRectangleWidget(inkRectangleWidget),
    inkVectorGraphicWidget(inkVectorGraphicWidget),
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

/// see [NativeDB](https://nativedb.red4ext.com/inkanimAnimationLibraryResource)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct inkanimAnimationLibraryResource {
    depot_path: PathBuf,
    flags: Flags,
}

/// see [NativeDB](https://nativedb.red4ext.com/inkWidgetLibraryResource)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
#[serde(rename_all = "camelCase")]
pub struct inkWidgetLibraryResource {
    pub animation_library_res_ref: inkanimAnimationLibraryResource,
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
}
