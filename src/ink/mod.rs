use clap::{builder::PossibleValue, ValueEnum};
use serde::{Deserialize, Deserializer, Serialize};
use serde_aux::prelude::*;

pub mod anim;
pub mod widget;

pub use anim::*;
pub use widget::*;

const OPACITY: InkAnimInterpolatorType = InkAnimInterpolatorType::Transparency(None);
const FADEIN: InkAnimInterpolatorType = InkAnimInterpolatorType::Transparency(Some(Fade::In));
const FADEOUT: InkAnimInterpolatorType = InkAnimInterpolatorType::Transparency(Some(Fade::Out));

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Fade {
    In,
    Out,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InkAnimInterpolatorType {
    Color,
    Size,
    Scale,
    Translation,
    Transparency(Option<Fade>),
    TextValueProgress,
    Effect,
    Anchor,
    Pivot,
    Shear,
    Rotation,
    Margin,
    Padding,
    TextReplace,
    TextOffset,
}

impl ValueEnum for InkAnimInterpolatorType {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Color,
            Self::Size,
            Self::Scale,
            Self::Translation,
            OPACITY,
            FADEIN,
            FADEOUT,
            Self::TextValueProgress,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Self::Color => Some(PossibleValue::new("color").alias("colour")),
            Self::Size => Some(PossibleValue::new("size").alias("dimension")),
            Self::Scale => Some(PossibleValue::new("scale")),
            Self::Translation => Some(PossibleValue::new("translation").alias("move")),
            Self::Transparency(direction) => match direction {
                None => Some(PossibleValue::new("transparency").aliases(["opacity", "alpha"])),
                Some(Fade::In) => Some(PossibleValue::new("fadein").aliases(["fade-in", "appear"])),
                Some(Fade::Out) => {
                    Some(PossibleValue::new("fadeout").aliases(["fade-out", "disappear"]))
                }
            },
            Self::TextValueProgress => {
                Some(PossibleValue::new("text-value-progress").aliases(["progress", "tvp"]))
            }
            Self::Effect => Some(PossibleValue::new("effect")),
            Self::Anchor => Some(PossibleValue::new("anchor")),
            Self::Pivot => Some(PossibleValue::new("pivot")),
            Self::Shear => Some(PossibleValue::new("shear")),
            Self::Rotation => Some(PossibleValue::new("rotation")),
            Self::Margin => Some(PossibleValue::new("margin")),
            Self::Padding => Some(PossibleValue::new("padding")),
            Self::TextReplace => {
                Some(PossibleValue::new("text-replace").aliases(["replace", "tr"]))
            }
            Self::TextOffset => Some(PossibleValue::new("text-offset").aliases(["offset", "to"])),
        }
    }
}

pub fn deserialize_handle_id_from_string<'de, D>(deserializer: D) -> Result<HandleId, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(HandleId(deserialize_number_from_string(deserializer)?))
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HandleId(u32);

pub struct OrphanInkAnimInterpolator {
    pub index: usize,
    pub interpolator: InkWrapper<InkAnimInterpolator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InkWrapper<T> {
    #[serde(deserialize_with = "deserialize_handle_id_from_string")]
    pub handle_id: HandleId,
    pub data: T,
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
