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
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// .inkwidget path
    #[arg(short, long)]
    pub widget: PathBuf,

    /// .inkanim path
    ///
    /// note: if left unspecified, it defaults to the same path as the .inkwidget,
    /// with suffix "_animations" and .inkanim extension instead
    #[arg(short, long)]
    pub anim: Option<PathBuf>,

    /// filter by path
    ///
    /// e.g. "1.3.0.0.16"
    #[arg(short, long, value_parser = parse_path_var)]
    pub path: Option<std::vec::Vec<usize>>,

    /// filter by interpolation type
    #[arg(short, long)]
    pub r#type: Option<InkAnimInterpolatorType>,

    /// show widgets name instead of index
    ///
    /// note: displaying names tend to disrupt CLI display (when too long)
    #[arg(short, long = "show", default_value_t = false)]
    pub show_path_names: bool,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Parser)] // requires `derive` feature
#[command(name = "inkanim")]
#[command(bin_name = "inkanim")]
pub enum CLI {
    List(Args),
}

fn parse_path_var(path: &str) -> Result<Vec<usize>, std::io::Error> {
    Ok(path
        .split('.')
        .into_iter()
        .map(|x| x.trim().parse::<usize>().expect("digit"))
        .collect())
}
