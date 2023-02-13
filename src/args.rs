use std::path::PathBuf;

use clap::{builder::PossibleValue, ValueEnum};

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
                Some(PossibleValue::new("text-value-progress").aliases(["text", "progress", "tvp"]))
            }
        }
    }
}

#[derive(clap::Args, Debug)]
pub struct Files {
    /// .inkwidget path
    #[arg(short, long, value_name = "FILE")]
    pub widget: PathBuf,

    /// optional .inkanim path
    ///
    /// note: if left unspecified, it defaults to the same path as the .inkwidget,
    /// with suffix "_animations" and .inkanim extension instead
    #[arg(short, long, value_name = "FILE")]
    pub anim: Option<PathBuf>,
}

#[derive(clap::Args, Debug)]
pub struct PathIndexes {
    /// filter by widget path indexes
    ///
    /// e.g. "1.3.0.0.16"
    #[arg(short, long, value_parser = parse_path_indexes, value_name = "INDEXES")]
    pub path: std::vec::Vec<usize>,
}

#[derive(clap::Args, Debug)]
pub struct OptionalPathIndexes {
    /// filter by widget path indexes
    ///
    /// e.g. "1.3.0.0.16"
    #[arg(short, long, value_parser = parse_path_indexes, value_name = "INDEXES")]
    pub path: Option<std::vec::Vec<usize>>,
}

#[derive(clap::Args, Debug)]
pub struct PathNames {
    /// filter by widget path name(s)
    ///
    /// e.g. "main_canvas.Arrival.Arrival_GPS_Canvas.Arrival_GPS_Elements_Canvas"
    #[arg(short, long, value_parser = parse_path_names, value_name = "NAMES")]
    pub path: std::vec::Vec<String>,
}

pub(crate) fn parse_path_indexes(path: &str) -> Result<Vec<usize>, std::io::Error> {
    Ok(path
        .split('.')
        .into_iter()
        .map(|x| x.trim().parse::<usize>().expect("digit"))
        .collect())
}

pub(crate) fn parse_path_names(path: &str) -> Result<Vec<String>, std::io::Error> {
    Ok(path
        .split('.')
        .into_iter()
        .map(|x| x.trim().to_string())
        .collect())
}
