use crate::args::{Files, InkAnimInterpolatorType, PathIndexes};

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(flatten)]
    pub files: Files,

    #[command(flatten)]
    pub path: Option<PathIndexes>,

    /// optionally filter by interpolation type
    #[arg(short, long)]
    pub r#type: Option<InkAnimInterpolatorType>,

    /// optionally show widgets name instead of index
    ///
    /// note: displaying names tend to disrupt CLI display (when too long)
    #[arg(short, long = "show", default_value_t = false)]
    pub show_path_names: bool,
}
