use crate::args::{Files, PathNames};

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(flatten)]
    pub files: Files,

    #[command(flatten)]
    pub names: PathNames,
}
