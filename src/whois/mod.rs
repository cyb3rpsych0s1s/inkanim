mod args;
pub(crate) use args::Args;

use crate::ink::{inkWidgetLibraryResource, InkAnimAnimationLibraryResource};

pub(crate) fn whois(
    args: Args,
    _: inkWidgetLibraryResource,
    anim: InkAnimAnimationLibraryResource,
) {
    let indexes = args.path.path;
    for sequence in anim.sequences {
        println!("{}", sequence.data.name);
        let summary = sequence.data.get_path_indexes_matching(&indexes);
        println!("{:#?}", summary);
    }
}
