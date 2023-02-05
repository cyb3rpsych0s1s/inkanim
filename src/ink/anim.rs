use std::fmt;

use serde::{
    de::{self, MapAccess},
    Deserialize, Serialize,
};
use serde_aux::prelude::*;

use super::InkWrapper;

fn deserialize_vector2_from_anything<'de, D>(deserializer: D) -> Result<Range, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct RangeVisitor;

    impl<'de> de::Visitor<'de> for RangeVisitor {
        type Value = Range;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("either a Vector2, or its simpler integer or float representation")
        }

        fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            println!("visit_u8");
            self.visit_i32(v as i32)
        }

        fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            println!("visit_u16");
            self.visit_i32(v as i32)
        }

        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_i32(v as i32)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_u32(v as u32)
        }

        fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_u32(v as u32)
        }

        fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_i32(v as i32)
        }

        fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_i32(v as i32)
        }

        fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Range::Position(Vector2 {
                x: v as f32,
                y: v as f32,
            }))
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_i32(v as i32)
        }

        fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_i32(v as i32)
        }

        fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Range::Position(Vector2 { x: v, y: v }))
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_f32(v as f32)
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let (key, value) = &mut map.next_entry::<&str, &str>()?.unwrap();
            if key == &"$type" {
                if value == &"Vector2" {
                    return Ok(Range::Position(Vector2::deserialize(
                        de::value::MapAccessDeserializer::new(map),
                    )?));
                }
                if value == &"HDRColor" {
                    return Ok(Range::Color(HDRColor::deserialize(
                        de::value::MapAccessDeserializer::new(map),
                    )?));
                }
            }
            Err(de::Error::custom("unknown type"))
        }
    }

    deserializer.deserialize_any(RangeVisitor)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Direction {
    To = 0,
    From = 1,
    FromTo = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Mode {
    EasyIn = 0,
    EasyOut = 1,
    EasyInOut = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
#[serde(rename_all = "PascalCase")]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
#[serde(rename_all = "PascalCase")]
pub struct HDRColor {
    pub alpha: f32,
    pub blue: f32,
    pub green: f32,
    pub red: f32,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Range {
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
}

#[allow(dead_code, non_snake_case)]
#[derive(Debug)]
pub struct PathSummary {
    HandleId: u32,
    Index: usize,
    Path: Vec<usize>,
}
