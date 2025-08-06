use crate::args::{Files, Mode, PathNames};

#[derive(clap::Args, Debug)]
#[command()]
pub struct Args {
    #[command(flatten)]
    pub files: Files,

    #[command(flatten)]
    pub names: PathNames,

    #[command(flatten)]
    pub mode: Mode,
}
