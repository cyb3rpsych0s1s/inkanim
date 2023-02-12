#![allow(dead_code, unused_variables)]

// use std::path::PathBuf;

// use args::Args;
// use clap::Command;
// use ink::InkAnimAnimationLibraryResource;

// use crate::ink::inkWidgetLibraryResource;

use std::path::PathBuf;

use clap::Parser;
use ink::inkWidgetLibraryResource;

use crate::{args::CLI, ink::InkAnimAnimationLibraryResource};

mod args;
mod ink;

fn main() {
    let args = CLI::parse();
    let args = match args {
        CLI::List(args) => args,
    };

    let widget_json_path = args.widget.clone();
    let anim_json_path = args.anim.unwrap_or_else(|| {
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
    let anim_json_export =
        std::fs::read_to_string(PathBuf::from(anim_json_path)).expect(".inkanim");

    let widget = serde_json::from_str::<inkWidgetLibraryResource>(&widget_json_export).unwrap();
    let mut anim =
        serde_json::from_str::<InkAnimAnimationLibraryResource>(&anim_json_export).unwrap();

    let filter_by_path = args.path.and_then(|x| {
        Some(
            x.split_whitespace()
                .map(|x| x.parse::<u32>().expect("digit"))
                .collect::<Vec<u32>>(),
        )
    });
    let filter_by_type = args.r#type;

    // // 1 3 0 0 is Booting_Screen
    // // 1 3 0 0 12 is rectangles_Beauty
    // let path = vec![1, 3, 0, 0, 9];
    // let anim = serde_json::from_str::<InkAnimAnimationLibraryResource>(&anim_json_export).unwrap();
    // let widget = serde_json::from_str::<inkWidgetLibraryResource>(&widget_json_export).unwrap();
    println!(
        "{}",
        anim.sequences
            .iter_mut()
            .map(|x| format!(
                "{} {}",
                x.data.name,
                x.data
                    .definitions
                    .iter_mut()
                    .filter_map(|x| {
                        if let Some(ref filter) = filter_by_type {
                            let interpolators: Vec<ink::InkWrapper<ink::InkAnimInterpolator>> = x
                                .data
                                .interpolators
                                .clone()
                                .into_iter()
                                .filter(|x| x.data == *filter)
                                .collect();
                            if interpolators.len() == 0 {
                                return None;
                            }
                            x.data.interpolators = interpolators;
                            return Some(x);
                        }
                        Some(x)
                    })
                    .map(|x| format!("{x}"))
                    .collect::<Vec<String>>()
                    .join("\n")
            ))
            .collect::<Vec<String>>()
            .join("\n\n")
    );
    // let matches = anim.sequences[0].data.get_path_indexes_matching(&path);
    // // println!("{matches:#?}");
    // // println!(
    // //     "found {} target(s) at this path or nested below",
    // //     matches.len()
    // // );
    // // println!("{widget:#?}");
    // let names = widget
    //     .library_items
    //     .get(0)
    //     .unwrap()
    //     .package
    //     .file
    //     .data
    //     .root_chunk
    //     .get_path_names(&path);
    // println!("names found in path {:#?}\n{:#?}", path, names);
}
