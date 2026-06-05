mod args;

pub use args::Args;
use inkanim_types::widget::inkWidgetLibraryResource;

pub(crate) fn generate(args: Args, widget: inkWidgetLibraryResource) {
    let Args { .. } = args;
    let root_chunk = widget.root_chunk();
    let _root = &root_chunk.root_widget.data;
}
