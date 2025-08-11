mod args;
pub(crate) use args::Args;

use inkanim_types::widget::{ByName, Widget, inkWidgetLibraryResource};

pub(crate) fn show(args: Args, widget: inkWidgetLibraryResource) {
    let names = args
        .names
        .path
        .iter()
        .map(|x| x.as_str())
        .collect::<Vec<_>>();
    if names.is_empty() {
        panic!("please specify widget path names");
    }
    let mut widget = Widget::inkCanvasWidget(widget.root_chunk().root_widget.data.clone());
    let mut indexes = Vec::with_capacity(names.len());
    for (idx, name) in names.iter().enumerate() {
        indexes.push(idx);
        (_, widget) = widget.by_name(name).unwrap_or_else(|| {
            panic!(
                "could not find {name} at {}",
                indexes
                    .iter()
                    .cloned()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" . ")
            )
        });
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&widget).expect("serialize back to JSON")
    );
}
