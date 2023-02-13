mod conversion;
mod display;

use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::args::{Fade, InkAnimInterpolatorType};

use super::{HandleId, InkWrapper};

use conversion::deserialize_vector2_from_anything;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Direction {
    To = 0,
    From = 1,
    FromTo = 2,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Mode {
    EasyIn = 0,
    EasyOut = 1,
    EasyInOut = 2,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Type {
    Linear = 0,
    Quadratic = 1,
    Qubic = 2,
    Quartic = 3,
    Quintic = 4,
    Sinusoidal = 5,
    Exponential = 6,
    Elastic = 7,
    Circular = 8,
    Back = 9,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InkAnimSequenceTargetInfo {
    pub path: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(tag = "$type")]
#[serde(rename_all = "PascalCase")]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(tag = "$type")]
#[serde(rename_all = "PascalCase")]
pub struct HDRColor {
    pub alpha: f32,
    pub blue: f32,
    pub green: f32,
    pub red: f32,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum Range {
    Percent(f32),
    Position(Vector2),
    Color(HDRColor),
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interpolator {
    pub duration: f32,
    #[serde(deserialize_with = "deserialize_vector2_from_anything")]
    pub end_value: Range,
    pub interpolation_direction: Direction,
    pub interpolation_mode: Mode,
    pub interpolation_type: Type,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_additive: bool,
    pub start_delay: f32,
    #[serde(deserialize_with = "deserialize_vector2_from_anything")]
    pub start_value: Range,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub use_relative_duration: bool,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum InkAnimInterpolator {
    inkanimScaleInterpolator(Interpolator),
    inkanimTranslationInterpolator(Interpolator),
    inkanimTransparencyInterpolator(Interpolator),
    inkanimSizeInterpolator(Interpolator),
    inkanimColorInterpolator(Interpolator),
    inkanimTextValueProgressInterpolator(Interpolator),
}

impl InkAnimInterpolator {
    pub fn as_emoji(&self) -> &str {
        match self {
            InkAnimInterpolator::inkanimScaleInterpolator(interpolator) => "scale",
            InkAnimInterpolator::inkanimTranslationInterpolator(interpolator) => "translation",
            InkAnimInterpolator::inkanimTransparencyInterpolator(interpolator) => "transparency",
            InkAnimInterpolator::inkanimSizeInterpolator(interpolator) => "size",
            InkAnimInterpolator::inkanimColorInterpolator(interpolator) => "color",
            InkAnimInterpolator::inkanimTextValueProgressInterpolator(interpolator) => {
                "text value progress"
            }
        }
    }
    pub fn starts(&self) -> f32 {
        match self {
            InkAnimInterpolator::inkanimScaleInterpolator(interpolator)
            | InkAnimInterpolator::inkanimTranslationInterpolator(interpolator)
            | InkAnimInterpolator::inkanimTransparencyInterpolator(interpolator)
            | InkAnimInterpolator::inkanimSizeInterpolator(interpolator)
            | InkAnimInterpolator::inkanimColorInterpolator(interpolator)
            | InkAnimInterpolator::inkanimTextValueProgressInterpolator(interpolator) => {
                interpolator.start_delay
            }
        }
    }
    pub fn ends(&self) -> f32 {
        match self {
            InkAnimInterpolator::inkanimScaleInterpolator(interpolator)
            | InkAnimInterpolator::inkanimTranslationInterpolator(interpolator)
            | InkAnimInterpolator::inkanimTransparencyInterpolator(interpolator)
            | InkAnimInterpolator::inkanimSizeInterpolator(interpolator)
            | InkAnimInterpolator::inkanimColorInterpolator(interpolator)
            | InkAnimInterpolator::inkanimTextValueProgressInterpolator(interpolator) => {
                self.starts() + interpolator.duration
            }
        }
    }
    pub fn direction(&self) -> Direction {
        match self {
            InkAnimInterpolator::inkanimScaleInterpolator(interpolator)
            | InkAnimInterpolator::inkanimTranslationInterpolator(interpolator)
            | InkAnimInterpolator::inkanimTransparencyInterpolator(interpolator)
            | InkAnimInterpolator::inkanimSizeInterpolator(interpolator)
            | InkAnimInterpolator::inkanimColorInterpolator(interpolator)
            | InkAnimInterpolator::inkanimTextValueProgressInterpolator(interpolator) => {
                interpolator.interpolation_direction
            }
        }
    }
    pub fn r#type(&self) -> Type {
        match self {
            InkAnimInterpolator::inkanimScaleInterpolator(interpolator)
            | InkAnimInterpolator::inkanimTranslationInterpolator(interpolator)
            | InkAnimInterpolator::inkanimTransparencyInterpolator(interpolator)
            | InkAnimInterpolator::inkanimSizeInterpolator(interpolator)
            | InkAnimInterpolator::inkanimColorInterpolator(interpolator)
            | InkAnimInterpolator::inkanimTextValueProgressInterpolator(interpolator) => {
                interpolator.interpolation_type
            }
        }
    }
    pub fn mode(&self) -> Mode {
        match self {
            InkAnimInterpolator::inkanimScaleInterpolator(interpolator)
            | InkAnimInterpolator::inkanimTranslationInterpolator(interpolator)
            | InkAnimInterpolator::inkanimTransparencyInterpolator(interpolator)
            | InkAnimInterpolator::inkanimSizeInterpolator(interpolator)
            | InkAnimInterpolator::inkanimColorInterpolator(interpolator)
            | InkAnimInterpolator::inkanimTextValueProgressInterpolator(interpolator) => {
                interpolator.interpolation_mode
            }
        }
    }
    pub fn duration(&self) -> f32 {
        match self {
            InkAnimInterpolator::inkanimScaleInterpolator(interpolator)
            | InkAnimInterpolator::inkanimTranslationInterpolator(interpolator)
            | InkAnimInterpolator::inkanimTransparencyInterpolator(interpolator)
            | InkAnimInterpolator::inkanimSizeInterpolator(interpolator)
            | InkAnimInterpolator::inkanimColorInterpolator(interpolator)
            | InkAnimInterpolator::inkanimTextValueProgressInterpolator(interpolator) => {
                interpolator.duration
            }
        }
    }
}

impl PartialEq<InkAnimInterpolatorType> for InkAnimInterpolator {
    fn eq(&self, other: &InkAnimInterpolatorType) -> bool {
        match self {
            Self::inkanimScaleInterpolator(_) if other == &InkAnimInterpolatorType::Scale => true,
            Self::inkanimTranslationInterpolator(_)
                if other == &InkAnimInterpolatorType::Translation =>
            {
                true
            }
            Self::inkanimTransparencyInterpolator(ref interpolator) => match other {
                InkAnimInterpolatorType::Transparency(None) => true,
                InkAnimInterpolatorType::Transparency(Some(Fade::In))
                    if interpolator.start_value < interpolator.end_value =>
                {
                    true
                }
                InkAnimInterpolatorType::Transparency(Some(Fade::Out))
                    if interpolator.start_value > interpolator.end_value =>
                {
                    true
                }
                _ => false,
            },
            Self::inkanimSizeInterpolator(_) if other == &InkAnimInterpolatorType::Size => true,
            Self::inkanimColorInterpolator(_) if other == &InkAnimInterpolatorType::Color => true,
            Self::inkanimTextValueProgressInterpolator(_)
                if other == &InkAnimInterpolatorType::TextValueProgress =>
            {
                true
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InkAnimDefinition {
    pub interpolators: Vec<InkWrapper<InkAnimInterpolator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InkAnimSequence {
    pub definitions: Vec<InkWrapper<InkAnimDefinition>>,
    pub name: String,
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InkAnimAnimationLibraryResource {
    pub cooking_platform: String,
    pub sequences: Vec<InkWrapper<InkAnimSequence>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlankInkAnimSequenceTargetInfo {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub handle_ref_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Target {
    WithHandleId(InkWrapper<InkAnimSequenceTargetInfo>),
    WithoutHandleId(BlankInkAnimSequenceTargetInfo),
}

pub trait SameOrNested {
    fn same_or_nested(&self, searched: &[usize]) -> bool;
}

impl SameOrNested for Vec<usize> {
    fn same_or_nested(&self, searched: &[usize]) -> bool {
        let count_own = self.len();
        let count_searched = searched.len();
        if count_searched == 0 {
            return true;
        }
        let last_searched = count_searched - 1;
        for (i, path_index) in self.iter().enumerate() {
            if *path_index != searched[i] {
                return false;
            }
            if i == last_searched && count_own >= count_searched {
                return true;
            }
        }
        return false;
    }
}

impl InkAnimSequence {
    pub fn get_path_indexes_matching(&self, searched: &[usize]) -> Vec<PathSummary> {
        let count = searched.len();
        let last = count - 1;
        let mut out = vec![];
        for (target_index, target) in self.targets.iter().enumerate() {
            match target {
                Target::WithHandleId(ref handle) => {
                    let ref path = handle.data.path;
                    'inner: for (i, path_index) in searched.iter().enumerate() {
                        if path_index != &path[i] {
                            break 'inner;
                        }
                        if i == last {
                            let summary = PathSummary {
                                Name: self.name.clone(),
                                HandleId: handle.handle_id,
                                Index: target_index,
                                Path: path.clone(),
                            };
                            out.push(summary);
                            break 'inner;
                        }
                    }
                }
                _ => continue,
            }
        }
        return out;
    }
    pub fn get_interpolators_matching(
        &self,
        filter: &InkAnimInterpolatorType,
    ) -> Vec<InkWrapper<InkAnimInterpolator>> {
        self.definitions
            .get(0)
            .expect("at least one ink anim definition")
            .data
            .interpolators
            .clone()
            .into_iter()
            .filter(|x| x.data == *filter)
            .collect()
    }
}

#[allow(dead_code, non_snake_case)]
#[derive(Debug)]
pub struct PathSummary {
    Name: String,
    HandleId: HandleId,
    Index: usize,
    Path: Vec<usize>,
}
