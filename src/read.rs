use inkanim::{anim::InkAnimAnimationLibraryResource, widget::inkWidgetLibraryResource, File};

use crate::args::Files;

pub fn read(args: &Files) -> (inkWidgetLibraryResource, InkAnimAnimationLibraryResource) {
    let widget_json_path = args.widget.clone();
    let anim_json_path = args.anim.clone().unwrap_or_else(|| {
        let path = args.widget.clone();
        path.with_file_name(
            args.widget
                .file_name()
                .unwrap()
                .to_string_lossy()
                .replace("inkwidget", "inkanim"),
        )
    });

    let widget_json_export = std::fs::read_to_string(widget_json_path).expect(".inkwidget");
    let anim_json_export = std::fs::read_to_string(anim_json_path).expect(".inkanim");

    let widget_resource =
        serde_json::from_str::<File<inkWidgetLibraryResource>>(&widget_json_export).unwrap();
    let anim_resource =
        serde_json::from_str::<File<InkAnimAnimationLibraryResource>>(&anim_json_export).unwrap();

    let widget = widget_resource.data.root_chunk;
    let anim = anim_resource.data.root_chunk;

    if anim.sequences.len() != widget.library_items.len() {
        panic!("widget and anim lengths must match")
    }

    (widget, anim)
}
