use crate::args::{Files, Mode, PathIndexes};

#[derive(clap::Args, Debug)]
#[command()]
pub struct Args {
    #[command(flatten)]
    pub files: Files,

    #[command(flatten)]
    pub path: PathIndexes,

    #[command(flatten)]
    pub mode: Mode,
}
