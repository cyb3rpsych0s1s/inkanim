#![allow(dead_code, unused_variables)]

use std::path::PathBuf;

use ink::InkAnimAnimationLibraryResource;

use crate::ink::inkWidgetLibraryResource;

mod ink;

fn main() {
    let anim_json_export =
        std::fs::read_to_string(PathBuf::from("./inkanim_connect_to_girl.json".to_string()))
            .expect(".inkanim");
    let widget_json_export = std::fs::read_to_string(PathBuf::from(
        "./inkwidget_connect_to_girl.json".to_string(),
    ))
    .expect(".inkwidget");

    // 1 3 0 0 is Booting_Screen
    // 1 3 0 0 12 is rectangles_Beauty
    let path = vec![1, 3, 0, 0, 9];
    let anim = serde_json::from_str::<InkAnimAnimationLibraryResource>(&anim_json_export).unwrap();
    let widget = serde_json::from_str::<inkWidgetLibraryResource>(&widget_json_export).unwrap();
    println!(
        "{}",
        anim.sequences
            .iter()
            .map(|x| format!(
                "{} {}",
                x.data.name,
                x.data
                    .definitions
                    .iter()
                    .map(|x| format!("{x}"))
                    .collect::<Vec<String>>()
                    .join("\n")
            ))
            .collect::<Vec<String>>()
            .join("\n\n")
    );
    let matches = anim.sequences[0].data.get_path_indexes_matching(&path);
    // println!("{matches:#?}");
    // println!(
    //     "found {} target(s) at this path or nested below",
    //     matches.len()
    // );
    // println!("{widget:#?}");
    let names = widget
        .library_items
        .get(0)
        .unwrap()
        .package
        .file
        .data
        .root_chunk
        .get_path_names(&path);
    println!("names found in path {:#?}\n{:#?}", path, names);
}
