mod args;
pub(crate) use args::Args;

use term_table::{
    Table, TableStyle,
    row::Row,
    table_cell::{Alignment, TableCell},
};

use inkanim_types::{
    anim::{InkAnimAnimationLibraryResource, OrphanInkAnimInterpolator, Target},
    widget::{SiblingOrNested, WidgetTree, inkWidgetLibraryResource},
};

use crate::args::DualResources;

const COLUMN_COUNT: usize = 11;

pub(crate) fn list(
    args: Args,
    widget: inkWidgetLibraryResource,
    anim: InkAnimAnimationLibraryResource,
) {
    let filter_by_path = args.path.path;
    let filter_by_type = args.r#type;
    let duo = DualResources {
        widget,
        anim,
        filter_by_path,
        filter_by_type,
        show_path_names: args.show_path_names,
    };

    match &args.mode.output {
        crate::args::Output::Table => {
            let tables: Vec<Table> = into_table(duo);
            println!(
                "{}",
                tables
                    .iter()
                    .map(|x| x.render())
                    .collect::<Vec<String>>()
                    .join("\n\n")
            );
        }
        crate::args::Output::Json => {
            let json = into_json(duo);
            println!("{json}");
        }
        crate::args::Output::Reds => todo!(),
    };
}

fn into_json(value: DualResources) -> String {
    serde_json::to_string_pretty(&value.widget).unwrap()
}
fn into_table(value: DualResources) -> Vec<Table> {
    let DualResources {
        widget,
        anim,
        show_path_names,
        filter_by_path,
        ..
    } = value;
    let mut tables: Vec<Table> = Vec::with_capacity(anim.sequences.len());
    let mut table: Table;
    let mut row: Row;
    for sequence in anim.sequences {
        table = Table::new();
        table.style = TableStyle::rounded();
        table.add_row(Row::new(vec![
            TableCell::builder(sequence.data.name.clone().as_str())
                .col_span(2)
                .alignment(Alignment::Center)
                .build(),
            TableCell::builder("index")
                .col_span(1)
                .alignment(Alignment::Center)
                .build(),
            TableCell::builder("kind")
                .col_span(1)
                .alignment(Alignment::Center)
                .build(),
            TableCell::builder("time")
                .col_span(4)
                .alignment(Alignment::Center)
                .build(),
            TableCell::builder("direction")
                .col_span(1)
                .alignment(Alignment::Center)
                .build(),
            TableCell::builder("interpolation")
                .col_span(1)
                .alignment(Alignment::Center)
                .build(),
            TableCell::builder("effect")
                .col_span(1)
                .alignment(Alignment::Center)
                .build(),
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
            if interpolators.is_empty() {
                continue;
            }
            let target = sequence
                .data
                .targets
                .get(idx_definition)
                .expect("each anim definition should have a corresponding widget target");
            let infos = match target {
                Target::WithHandleId(infos) => Some(infos.clone().data.path),
                Target::WithoutHandleId(_) => None,
            };
            if let Some(ref filter) = filter_by_path
                && infos
                    .as_ref()
                    .map(|x| !x.sibling_or_nested(filter))
                    .unwrap_or(true)
            {
                continue;
            }

            if show_path_names {
                let fqcn = infos.clone().and_then(|x| {
                    widget
                        .library_items
                        .first()
                        .expect("Root")
                        .package
                        .data
                        .file
                        .root_chunk
                        .get_path_names(&x)
                });
                table.add_row(Row::new(vec![
                    TableCell::new(idx_definition),
                    TableCell::new(definition.handle_id),
                    TableCell::builder(
                        fqcn.map(|x| x.join(" . "))
                            .unwrap_or_else(|| "".to_string()),
                    )
                    .col_span(COLUMN_COUNT - 2)
                    .alignment(Alignment::Left)
                    .build(),
                ]));
            } else {
                table.add_row(Row::new(vec![
                    TableCell::new(idx_definition),
                    TableCell::new(definition.handle_id),
                    TableCell::builder(
                        infos
                            .clone()
                            .map(|x| {
                                x.iter()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<_>>()
                                    .join(" . ")
                            })
                            .unwrap_or_else(|| "".to_string()),
                    )
                    .col_span(COLUMN_COUNT - 2)
                    .alignment(Alignment::Left)
                    .build(),
                ]));
            }
            let kind = infos.clone().and_then(|x| {
                widget
                    .library_items
                    .first()
                    .expect("Root")
                    .package
                    .data
                    .file
                    .root_chunk
                    .get_widget_classname(&x)
            });

            for (idx_orphan, orphan) in interpolators.into_iter().enumerate() {
                if idx_orphan == 0 {
                    row = Row::new(vec![
                        TableCell::builder(
                            kind.as_ref()
                                .map(|x| x.to_string())
                                .unwrap_or_else(|| "".to_string())
                                .as_str(),
                        )
                        .col_span(2)
                        .build(),
                    ]);
                } else {
                    row = Row::new(vec![TableCell::builder("").col_span(2).build()]);
                }
                row.has_separator = idx_orphan == 0;
                row.cells.push(
                    TableCell::builder(orphan.index)
                        .col_span(1)
                        .alignment(Alignment::Center)
                        .build(),
                );
                row.cells.push(
                    TableCell::builder(orphan.interpolator.data.as_short_display())
                        .col_span(1)
                        .alignment(Alignment::Center)
                        .build(),
                );
                row.cells.push(
                    TableCell::builder(orphan.interpolator.data.starts())
                        .col_span(1)
                        .alignment(Alignment::Left)
                        .build(),
                );
                row.cells.push(TableCell::builder("=>").col_span(1).build());
                row.cells.push(
                    TableCell::builder(orphan.interpolator.data.ends())
                        .col_span(1)
                        .alignment(Alignment::Left)
                        .build(),
                );
                row.cells.push(
                    TableCell::builder(format!("({})", orphan.interpolator.data.duration(),))
                        .col_span(1)
                        .alignment(Alignment::Left)
                        .build(),
                );
                row.cells.push(
                    TableCell::builder(format!("{}", orphan.interpolator.data.direction(),))
                        .col_span(1)
                        .alignment(Alignment::Center)
                        .build(),
                );
                row.cells.push(
                    TableCell::builder(format!("{}", orphan.interpolator.data.transformation(),))
                        .col_span(1)
                        .alignment(Alignment::Left)
                        .build(),
                );
                row.cells.push(
                    TableCell::builder(format!(
                        "{}.{}",
                        orphan.interpolator.data.r#type(),
                        orphan.interpolator.data.mode()
                    ))
                    .col_span(1)
                    .alignment(Alignment::Right)
                    .build(),
                );
                table.add_row(row.clone());
            }
        }
        tables.push(table);
    }
    tables
}
