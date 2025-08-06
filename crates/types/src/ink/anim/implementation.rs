use clap::{ValueEnum, builder::PossibleValue};

use super::{FADEIN, FADEOUT, Fade, InkAnimInterpolatorType, OPACITY};

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
