use std::path::PathBuf;

use crate::args::Files;

#[derive(clap::Args, Debug)]
#[command()]
pub struct Args {
    #[command(flatten)]
    pub files: Files,
    /// .reds output path
    #[arg(short, long, value_name = "OUTPUT_REDS")]
    pub reds: PathBuf,
}
