//! All widgets in Cyberpunk 2077 UI
//! are similar to web and traditional UI frameworks.

pub mod font;
pub mod image;
pub(crate) mod implementation;
pub mod layout;
pub mod properties;

use enum_dispatch::enum_dispatch;
pub use implementation::*;

use serde::{Deserialize, Serialize, ser::SerializeSeq};
use serde_aux::prelude::deserialize_bool_from_anything;

use crate::{DepotPath, Name, Vector2, is_any_default_localization_string};

use self::{
    font::{
        fontStyle, inkFontFamilyResource, textHorizontalAlignment, textLetterCase,
        textOverflowPolicy, textVerticalAlignment,
    },
    image::{inkBrushMirrorType, inkBrushTileType, inkTextureAtlas},
    layout::{inkEHorizontalAlign, inkEVerticalAlign, inkMargin, textJustificationType},
};

use super::{HandleId, InkWrapper, LocalizationString};

/// belongs to the same level or is nested below, in a tree hierarchy
pub trait SiblingOrNested {
    fn sibling_or_nested(&self, searched: &[usize]) -> bool;
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Flags {
    #[default]
    Default,
    Soft,
    Hard,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pivot(Vector2);

impl Default for Pivot {
    fn default() -> Self {
        Self(Vector2 { x: 0.5, y: 0.5 })
    }
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

macro_rules! native_compound_widget {
    ($ty:ident) => {
        #[doc=concat!("see [NativeDB](https://nativedb.red4ext.com/", stringify!($ty), ")")]
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct $ty {
            #[serde(default, skip_serializing_if = "is_default")]
            pub children: InkWrapper<inkMultiChildren>,
            #[serde(default, skip_serializing_if = "is_default")]
            pub name: $crate::Name,
            #[serde(default, skip_serializing_if = "is_default")]
            pub child_order: self::layout::inkEChildOrder,
            #[serde(default, skip_serializing_if = "is_default")]
            pub child_margin: self::layout::inkMargin,
        }
    };
}

macro_rules! native_leaf_widget {
    ($ty:ident { $($tt:tt)* }) => {
        #[doc=concat!("🌿 see [NativeDB](https://nativedb.red4ext.com/", stringify!($ty), ")")]
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct $ty {
            #[serde(default, skip_serializing_if = "is_default")]
            pub name: $crate::Name,
            #[serde(default, skip_serializing_if = "is_default")]
            pub layout: self::layout::inkWidgetLayout,
            #[serde(default, skip_serializing_if = "is_default")]
            pub property_manager: Option<self::properties::PropertyManager>,
            #[serde(default, skip_serializing_if = "is_default")]
            pub render_transform_pivot: self::Pivot,
            #[serde(default, skip_serializing_if = "is_default")]
            pub render_transform: self::layout::inkUITransform,
            #[serde(default, skip_serializing_if = "is_default")]
            pub size: crate::Vector2,
            $($tt)*
        }
    };
    ($ty:ident) => {
        native_leaf_widget!($ty {});
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
#[derive(Debug, Default, Clone, Deserialize, PartialEq)]
pub struct inkMultiChildren {
    pub children: Vec<InkWrapper<Widget>>,
}

impl Serialize for inkMultiChildren {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.children.len()))?;
        for elem in self.children.iter() {
            seq.serialize_element(elem)?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct ScrollDelay(u16);

impl Default for ScrollDelay {
    fn default() -> Self {
        Self(30)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct ScrollTextSpeed(f32);

impl Default for ScrollTextSpeed {
    fn default() -> Self {
        Self(0.2)
    }
}

native_leaf_widget!(inkTextWidget {
  #[serde(default, skip_serializing_if = "is_any_default_localization_string",)]
  pub localization_string: LocalizationString,
  #[serde(default, skip_serializing_if = "is_default")]
  pub text: String,
  #[serde(default, skip_serializing_if = "is_default")]
  pub font_family: inkFontFamilyResource,
  #[serde(default, skip_serializing_if = "is_default")]
  pub font_style: fontStyle,
  #[serde(default, skip_serializing_if = "is_default")]
  pub justification: textJustificationType,
  #[serde(default, skip_serializing_if = "is_default")]
  pub text_letter_case: Option<textLetterCase>,
  #[serde(default, skip_serializing_if = "is_default")]
  pub line_height_percentage: f32,
  #[serde(default, skip_serializing_if = "is_default")]
  pub text_horizontal_alignment: textHorizontalAlignment,
  #[serde(default, skip_serializing_if = "is_default")]
  pub text_vertical_alignment: textVerticalAlignment,
  #[serde(default, skip_serializing_if = "is_default")]
  pub text_overflow_policy: textOverflowPolicy,
  #[serde(default, skip_serializing_if = "is_default")]
  pub content_h_align: inkEHorizontalAlign,
  #[serde(default, skip_serializing_if = "is_default")]
  pub content_v_align: inkEVerticalAlign,
  #[serde(default, skip_serializing_if = "is_default")]
  pub scroll_delay: self::ScrollDelay,
  #[serde(default, skip_serializing_if = "is_default")]
  pub scroll_text_speed: self::ScrollTextSpeed,
});
native_leaf_widget!(inkImageWidget {
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub use_external_dynamic_texture: bool,
    pub external_dynamic_texture: Name,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub use_nine_slice_scale: bool,
    pub nine_slice_scale: inkMargin,
    pub mirror_type: inkBrushMirrorType,
    pub tile_type: inkBrushTileType,
    pub horizontal_tile_crop: f32,
    pub vertical_tile_crop: f32,
    pub texture_atlas: inkTextureAtlas,
    pub texture_part: Name,
    pub content_h_align: inkEHorizontalAlign,
    pub content_v_align: inkEVerticalAlign,
    pub tile_h_align: inkEHorizontalAlign,
    pub tile_v_align: inkEVerticalAlign,
});
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
#[serde(rename_all = "camelCase")]
pub struct inkWidgetLibraryItemInstance {
    pub root_widget: InkWrapper<inkCanvasWidget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub file: crate::Data<inkWidgetLibraryItemInstance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Package {
    pub data: self::Data,
}

/// see [NativeDB](https://nativedb.red4ext.com/inkWidgetLibraryItem)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct inkWidgetLibraryItem {
    pub name: Name,
    pub package: Package,
}

/// see [NativeDB](https://nativedb.red4ext.com/inkanimAnimationLibraryResource)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct inkanimAnimationLibraryResource {
    depot_path: DepotPath,
    flags: Flags,
}

/// see [NativeDB](https://nativedb.red4ext.com/inkWidgetLibraryResource)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub Name: Name,
}
