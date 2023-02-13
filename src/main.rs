#![allow(dead_code, unused_variables)]

// use std::path::PathBuf;

// use args::Args;
// use clap::Command;
// use ink::InkAnimAnimationLibraryResource;

// use crate::ink::inkWidgetLibraryResource;

use std::path::PathBuf;

use args::InkAnimInterpolatorType;
use clap::Parser;
use ink::{inkWidgetLibraryResource, InkAnimInterpolator, InkWrapper, SameOrNested};
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};

use crate::{args::CLI, ink::InkAnimAnimationLibraryResource};

mod args;
mod ink;

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
    // println!("{widget:#?}");

    let anim = serde_json::from_str::<InkAnimAnimationLibraryResource>(&anim_json_export).unwrap();

    let filter_by_path = args.path;
    let filter_by_type = args.r#type;
    let duo = DualResources {
        widget,
        anim,
        filter_by_path,
        filter_by_type,
        show_path_names: args.show_path_names,
    };

    let tables: Vec<Table> = duo.into();
    println!(
        "{}",
        tables
            .iter()
            .map(|x| x.render())
            .collect::<Vec<String>>()
            .join("\n\n")
    );

    // // 1 3 0 0 is Booting_Screen
    // // 1 3 0 0 12 is rectangles_Beauty
    // let path = vec![1, 3, 0, 0, 9];
    // let anim = serde_json::from_str::<InkAnimAnimationLibraryResource>(&anim_json_export).unwrap();
    // let widget = serde_json::from_str::<inkWidgetLibraryResource>(&widget_json_export).unwrap();
    // println!(
    //     "{}",
    //     anim.sequences
    //         .iter_mut()
    //         .map(|x| format!(
    //             "{} {}",
    //             x.data.name,
    //             x.data
    //                 .definitions
    //                 .iter_mut()
    //                 .filter_map(|x| {
    //                     if let Some(ref filter) = filter_by_type {
    //                         let interpolators: Vec<ink::InkWrapper<ink::InkAnimInterpolator>> = x
    //                             .data
    //                             .interpolators
    //                             .clone()
    //                             .into_iter()
    //                             .filter(|x| x.data == *filter)
    //                             .collect();
    //                         if interpolators.len() == 0 {
    //                             return None;
    //                         }
    //                         x.data.interpolators = interpolators;
    //                         return Some(x);
    //                     }
    //                     Some(x)
    //                 })
    //                 .map(|x| format!("{x}"))
    //                 .collect::<Vec<String>>()
    //                 .join("\n")
    //         ))
    //         .collect::<Vec<String>>()
    //         .join("\n\n")
    // );
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

impl<'a> From<DualResources> for Vec<Table<'a>> {
    fn from(value: DualResources) -> Self {
        let DualResources {
            widget,
            anim,
            filter_by_type,
            show_path_names,
            filter_by_path,
        } = value;
        if anim.sequences.len() != widget.library_items.len() {
            panic!("widget and anim lengths must match")
        }
        let mut tables: Vec<Table> = Vec::with_capacity(anim.sequences.len());
        let mut table: Table;
        let mut row: Row;
        // let target: T;
        for sequence in anim.sequences {
            table = Table::new();
            table.style = TableStyle::rounded();
            table.add_row(Row::new(vec![
                TableCell::new_with_alignment(sequence.data.name.clone(), 2, Alignment::Center),
                TableCell::new_with_alignment("index", 1, Alignment::Center),
                TableCell::new_with_alignment("kind", 1, Alignment::Center),
                TableCell::new_with_alignment("time", 4, Alignment::Center),
                TableCell::new_with_alignment("direction", 1, Alignment::Center),
                TableCell::new_with_alignment("effect", 1, Alignment::Center),
            ]));
            for (idx_definition, definition) in sequence.data.definitions.into_iter().enumerate() {
                let interpolators: Vec<OrphanInkAnimInterpolator> =
                    if let Some(ref filter) = value.filter_by_type {
                        definition
                            .data
                            .interpolators
                            .into_iter()
                            .enumerate()
                            .filter_map(|(idx, x)| {
                                if x.data == *filter {
                                    let orphan = OrphanInkAnimInterpolator {
                                        index: idx,
                                        interpolator: x,
                                    };
                                    return Some(orphan);
                                }
                                None
                            })
                            .collect()
                    } else {
                        definition
                            .data
                            .interpolators
                            .into_iter()
                            .enumerate()
                            .map(|(idx, x)| OrphanInkAnimInterpolator {
                                index: idx,
                                interpolator: x,
                            })
                            .collect()
                    };
                if interpolators.len() == 0 {
                    continue;
                }
                let target = sequence
                    .data
                    .targets
                    .get(idx_definition)
                    .expect("each anim definition should have a corresponding widget target");
                let infos = match target {
                    ink::Target::WithHandleId(infos) => Some(infos.clone().data.path),
                    ink::Target::WithoutHandleId(_) => None,
                };
                if let Some(ref filter) = filter_by_path {
                    if infos
                        .as_ref()
                        .map(|x| !x.same_or_nested(filter))
                        .unwrap_or(true)
                    {
                        continue;
                    }
                }

                if show_path_names {
                    let fqcn = infos.clone().and_then(|x| {
                        widget
                            .library_items
                            .get(0)
                            .expect("Root")
                            .package
                            .file
                            .data
                            .root_chunk
                            .get_path_names(&x)
                    });
                    table.add_row(Row::new(vec![
                        TableCell::new(idx_definition),
                        TableCell::new(definition.handle_id),
                        TableCell::new_with_alignment(
                            fqcn.and_then(|x| Some(x.join(" . ")))
                                .unwrap_or("".to_string()),
                            8,
                            Alignment::Left,
                        ),
                    ]));
                } else {
                    table.add_row(Row::new(vec![
                        TableCell::new(idx_definition),
                        TableCell::new(definition.handle_id),
                        TableCell::new_with_alignment(
                            infos
                                .and_then(|x| {
                                    Some(
                                        x.iter()
                                            .map(|x| x.to_string())
                                            .collect::<Vec<_>>()
                                            .join(" . "),
                                    )
                                })
                                .unwrap_or("".to_string()),
                            8,
                            Alignment::Left,
                        ),
                    ]));
                }

                for (idx_orphan, orphan) in interpolators.into_iter().enumerate() {
                    row = Row::new(vec![TableCell::new_with_col_span("", 2)]);
                    row.has_separator = idx_orphan == 0;
                    row.cells.push(TableCell::new_with_alignment(
                        orphan.index,
                        1,
                        Alignment::Center,
                    ));
                    row.cells.push(TableCell::new_with_alignment(
                        orphan.interpolator.data.as_emoji(),
                        1,
                        Alignment::Center,
                    ));
                    row.cells.push(TableCell::new_with_alignment(
                        orphan.interpolator.data.starts(),
                        1,
                        Alignment::Left,
                    ));
                    row.cells.push(TableCell::new_with_col_span("=>", 1));
                    row.cells.push(TableCell::new_with_alignment(
                        orphan.interpolator.data.ends(),
                        1,
                        Alignment::Left,
                    ));
                    row.cells.push(TableCell::new_with_alignment(
                        format!("({})", orphan.interpolator.data.duration(),),
                        1,
                        Alignment::Left,
                    ));
                    row.cells.push(TableCell::new_with_alignment(
                        format!("{}", orphan.interpolator.data.direction(),),
                        1,
                        Alignment::Center,
                    ));
                    row.cells.push(TableCell::new_with_alignment(
                        format!(
                            "{}.{}",
                            orphan.interpolator.data.r#type(),
                            orphan.interpolator.data.mode()
                        ),
                        1,
                        Alignment::Right,
                    ));
                    table.add_row(row.clone());
                }
            }
            tables.push(table);
        }
        tables
    }
}
