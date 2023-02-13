use args::InkAnimInterpolatorType;
use clap::Parser;
use cli::CLI;
use ink::{inkWidgetLibraryResource, InkAnimInterpolator, InkWrapper};

use crate::ink::InkAnimAnimationLibraryResource;

mod args;
mod cli;
mod ink;
mod list;
mod read;
mod whereis;
mod whois;

use list::list;
use read::read;
use whereis::whereis;
use whois::whois;

pub struct DualResources {
    pub widget: inkWidgetLibraryResource,
    pub anim: InkAnimAnimationLibraryResource,
    pub filter_by_path: Option<Vec<usize>>,
    pub filter_by_type: Option<InkAnimInterpolatorType>,
    pub show_path_names: bool,
}

pub struct OrphanInkAnimInterpolator {
    pub index: usize,
    pub interpolator: InkWrapper<InkAnimInterpolator>,
}

fn main() {
    let args = CLI::parse();
    let files = match args {
        CLI::List(list::Args { ref files, .. }) => files,
        CLI::WhoIs(whois::Args { ref files, .. }) => files,
        CLI::WhereIs(whereis::Args { ref files, .. }) => files,
    };
    let (widget, anim) = read(files);
    match args {
        CLI::List(args) => list(args, widget, anim),
        CLI::WhoIs(args) => whois(args, widget, anim),
        CLI::WhereIs(args) => whereis(args, widget, anim),
    };
}
