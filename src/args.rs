use std::path::PathBuf;

use clap::{builder::PossibleValue, Parser, ValueEnum};

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

const OPACITY: InkAnimInterpolatorType = InkAnimInterpolatorType::Transparency(None);
const FADEIN: InkAnimInterpolatorType = InkAnimInterpolatorType::Transparency(Some(Fade::In));
const FADEOUT: InkAnimInterpolatorType = InkAnimInterpolatorType::Transparency(Some(Fade::Out));

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
            InkAnimInterpolatorType::Color => Some(PossibleValue::new("color").alias("colour")),
            InkAnimInterpolatorType::Size => Some(PossibleValue::new("size").alias("dimension")),
            InkAnimInterpolatorType::Scale => Some(PossibleValue::new("scale")),
            InkAnimInterpolatorType::Translation => {
                Some(PossibleValue::new("translation").alias("move"))
            }
            InkAnimInterpolatorType::Transparency(direction) => match direction {
                None => Some(PossibleValue::new("transparency").aliases(["opacity", "alpha"])),
                Some(Fade::In) => Some(PossibleValue::new("fadein").aliases(["fade-in", "appear"])),
                Some(Fade::Out) => {
                    Some(PossibleValue::new("fadeout").aliases(["fade-out", "disappear"]))
                }
            },
            InkAnimInterpolatorType::TextValueProgress => {
                Some(PossibleValue::new("text-value-progress").aliases(["text", "progress", "tvp"]))
            }
        }
    }
}

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// .inkwidget path
    #[arg(short, long)]
    pub widget: PathBuf,

    /// .inkanim path
    ///
    /// if left unspecified, it defaults to the same path as the .inkwidget,
    /// with suffix "_animations" and .inkanim extension instead
    #[arg(short, long)]
    pub anim: Option<PathBuf>,

    /// filter by path
    ///
    /// e.g. "1 3 0 0 16"
    #[arg(short, long)]
    pub path: Option<String>,

    /// filter by interpolation type
    ///
    /// e.g. "scale", "transparency", "translation", and so on
    #[arg(short, long)]
    pub r#type: Option<InkAnimInterpolatorType>,
}

#[derive(Parser)] // requires `derive` feature
#[command(name = "ink")]
#[command(bin_name = "ink")]
pub enum CLI {
    List(Args),
}
