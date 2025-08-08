mod args;
pub(crate) use args::Args;

use inkanim_types::{
    File, InkWrapper,
    widget::{InkChildren, Widget, inkWidgetLibraryResource},
};

pub(crate) fn tree(_: Args, widget: File<inkWidgetLibraryResource>) {
    println!("---\n{}\n---", widget.header);
    let widget = widget.resource();
    for child in widget.root_chunk().root_widget.children() {
        process(&child, 0);
    }
}

fn process(child: &InkWrapper<Widget>, mut indent: usize) {
    println!(
        "{}{}{} => {}",
        "\t".repeat(indent),
        if child.data.is_leaf() {
            "🌿"
        } else {
            "🪵 "
        },
        child.handle_id,
        child.data.name().unwrap_or("unknown")
    );
    if let Some(compound) = child.data.as_compound() {
        indent += 1;
        for child in compound.children() {
            process(&child, indent);
        }
    }
}
