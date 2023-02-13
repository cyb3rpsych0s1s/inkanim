use crate::args::{Files, PathNames};

#[derive(clap::Args, Debug)]
#[command()]
pub struct Args {
    #[command(flatten)]
    pub files: Files,

    #[command(flatten)]
    pub names: PathNames,
}
