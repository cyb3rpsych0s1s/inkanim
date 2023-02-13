use super::{
    Direction, HDRColor, InkAnimDefinition, InkAnimInterpolator, Interpolator, Mode, Range, Type,
    Vector2,
};

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl std::fmt::Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "X: {}, Y: {}", self.x, self.y)
    }
}

impl std::fmt::Display for HDRColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "R:{}, G: {}, B: {}, A: {}",
            self.red, self.green, self.blue, self.alpha
        )
    }
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Position(position) => write!(f, "{position}"),
            Self::Color(color) => write!(f, "{color}"),
            Self::Percent(percent) => write!(f, "{percent}"),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl std::fmt::Display for Interpolator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl std::fmt::Display for InkAnimInterpolator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl std::fmt::Display for InkAnimDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
