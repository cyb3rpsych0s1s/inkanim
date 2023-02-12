use std::fmt;

use serde::{
    de::{self, MapAccess},
    Deserialize, Serialize,
};
use serde_aux::prelude::*;
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};

use crate::args::{Fade, InkAnimInterpolatorType};

use super::{HandleId, InkWrapper};

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
            self.visit_i32(v as i32)
        }

        fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
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
            Ok(Range::Percent(v as f32))
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
            Ok(Range::Percent(v))
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Direction {
    To = 0,
    From = 1,
    FromTo = 2,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::To => "To",
                Self::From => "From",
                Self::FromTo => "FromTo",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Mode {
    EasyIn = 0,
    EasyOut = 1,
    EasyInOut = 2,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::EasyIn => "EasyIn",
                Self::EasyOut => "EasyOut",
                Self::EasyInOut => "EasyInOut",
            }
        )
    }
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

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Linear => "Linear",
                Self::Quadratic => "Quadratic",
                Self::Qubic => "Qubic",
                Self::Quartic => "Quartic",
                Self::Quintic => "Quintic",
                Self::Sinusoidal => "Sinusoidal",
                Self::Exponential => "Exponential",
                Self::Elastic => "Elastic",
                Self::Circular => "Circular",
                Self::Back => "Back",
            }
        )
    }
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

impl std::fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "X: {}, Y: {}", self.x, self.y)
    }
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

impl std::fmt::Display for HDRColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "R:{}, G: {}, B: {}, A: {}",
            self.red, self.green, self.blue, self.alpha
        )
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum Range {
    Percent(f32),
    Position(Vector2),
    Color(HDRColor),
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Range::Position(position) => write!(f, "{position}"),
            Range::Color(color) => write!(f, "{color}"),
            Range::Percent(percent) => write!(f, "{percent}"),
        }
    }
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

impl std::fmt::Display for Interpolator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} => {} starts at {}, until {} (duration: {}, relative: {})",
            self.start_value,
            self.end_value,
            self.start_delay,
            self.start_delay + self.duration,
            self.duration,
            self.use_relative_duration
        )
    }
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

impl std::fmt::Display for InkAnimInterpolator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InkAnimInterpolator::inkanimScaleInterpolator(interpolator) => {
                write!(f, "{} {}", "♻️", interpolator)
            }
            InkAnimInterpolator::inkanimTranslationInterpolator(interpolator) => {
                write!(f, "{} {}", "↕️", interpolator)
            }
            InkAnimInterpolator::inkanimTransparencyInterpolator(interpolator) => {
                write!(f, "{} {}", "👻", interpolator)
            }
            InkAnimInterpolator::inkanimSizeInterpolator(interpolator) => {
                write!(f, "{} {}", "📏", interpolator)
            }
            InkAnimInterpolator::inkanimColorInterpolator(interpolator) => {
                write!(f, "{} {}", "🎨", interpolator)
            }
            InkAnimInterpolator::inkanimTextValueProgressInterpolator(interpolator) => {
                write!(f, "{} {:#?}", "🈺", interpolator)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InkAnimDefinition {
    pub interpolators: Vec<InkWrapper<InkAnimInterpolator>>,
}

impl std::fmt::Display for InkAnimDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.interpolators
                .iter()
                .enumerate()
                .map(|(idx, x)| { format!("[{idx}] {x}") })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
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

impl<'a> From<InkAnimAnimationLibraryResource> for Vec<Table<'a>> {
    fn from(value: InkAnimAnimationLibraryResource) -> Self {
        let mut tables: Vec<Table> = Vec::with_capacity(value.sequences.len());
        let mut table: Table;
        let mut row: Row;
        for sequence in value.sequences {
            table = Table::new();
            table.style = TableStyle::extended();
            table.add_row(Row::new(vec![
                TableCell::new_with_alignment(sequence.data.name.clone(), 2, Alignment::Center),
                TableCell::new_with_alignment("index", 1, Alignment::Center),
                TableCell::new_with_alignment("kind", 1, Alignment::Center),
                TableCell::new_with_alignment("starts at", 1, Alignment::Center),
                TableCell::new_with_alignment("ends at", 1, Alignment::Center),
                TableCell::new_with_alignment("direction", 1, Alignment::Center),
                TableCell::new_with_alignment("effect", 1, Alignment::Center),
            ]));
            for (idx_definition, definition) in sequence.data.definitions.into_iter().enumerate() {
                for (idx_interpolator, interpolator) in
                    definition.data.interpolators.into_iter().enumerate()
                {
                    if idx_interpolator == 0 {
                        row = Row::new(vec![
                            TableCell::new(idx_definition),
                            TableCell::new(definition.handle_id),
                        ]);
                    } else {
                        row = Row::new(vec![TableCell::new_with_col_span("", 2)]);
                    }
                    row.cells.push(TableCell::new_with_alignment(
                        idx_interpolator,
                        1,
                        Alignment::Center,
                    ));
                    row.cells.push(TableCell::new_with_alignment(
                        interpolator.data.as_emoji(),
                        1,
                        Alignment::Center,
                    ));
                    row.cells.push(TableCell::new_with_alignment(
                        interpolator.data.starts(),
                        1,
                        Alignment::Center,
                    ));
                    row.cells.push(TableCell::new_with_alignment(
                        interpolator.data.ends(),
                        1,
                        Alignment::Center,
                    ));
                    row.cells.push(TableCell::new_with_alignment(
                        format!("{}", interpolator.data.direction(),),
                        1,
                        Alignment::Center,
                    ));
                    row.cells.push(TableCell::new_with_alignment(
                        format!(
                            "{}.{}",
                            interpolator.data.r#type(),
                            interpolator.data.mode()
                        ),
                        1,
                        Alignment::Right,
                    ));
                    table.add_row(row.clone());
                }
            }
            tables.push(table);
        }
        tables
    }
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
