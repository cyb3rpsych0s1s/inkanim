mod args;
pub(crate) use args::Args;
use inkanim::{anim::InkAnimAnimationLibraryResource, widget::inkWidgetLibraryResource};

pub(crate) fn show(
    args: Args,
    _widget: inkWidgetLibraryResource,
    _anim: InkAnimAnimationLibraryResource,
) {
    match (args.indexes.path, args.names.path) {
        (None, None) | (Some(_), Some(_)) => panic!("please specify either --indexes or --names"),
        (None, Some(_names)) => todo!(),
        (Some(_indexes), None) => todo!(),
    }
}
