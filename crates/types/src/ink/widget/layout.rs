use inkanim_macros::Reds;
use serde::{Deserialize, Serialize};

use crate::Vector2;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, Reds)]
pub enum inkEAnchor {
    TopLeft = 0,
    TopCenter = 1,
    TopRight = 2,
    CenterLeft = 3,
    Centered = 4,
    CenterRight = 5,
    BottomLeft = 6,
    BottomCenter = 7,
    BottomRight = 8,
    TopFillHorizontaly = 9,
    CenterFillHorizontaly = 10,
    BottomFillHorizontaly = 11,
    LeftFillVerticaly = 12,
    CenterFillVerticaly = 13,
    RightFillVerticaly = 14,
    Fill = 15,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, Reds)]
pub enum inkEHorizontalAlign {
    Fill = 0,
    Left = 1,
    Center = 2,
    Right = 3,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, Reds)]
pub enum inkEVerticalAlign {
    Fill = 0,
    Top = 1,
    Center = 2,
    Bottom = 3,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct inkUITransform {
    pub translation: Vector2,
    pub scale: Vector2,
    pub shear: Vector2,
    pub rotation: f32,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, Reds)]
pub enum textJustificationType {
    Left = 0,
    Center = 1,
    Right = 2,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub struct inkMargin {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type", rename_all = "camelCase")]
pub struct inkWidgetLayout {
    pub anchor: inkEAnchor,
    pub anchor_point: Vector2,
    pub padding: inkMargin,
    pub margin: inkMargin,
    #[serde(rename = "HAlign")]
    pub h_align: inkEHorizontalAlign,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, Reds)]
pub enum inkEChildOrder {
    Forward = 0,
    Backward = 1,
}
