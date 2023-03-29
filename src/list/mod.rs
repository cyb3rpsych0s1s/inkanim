mod args;
pub(crate) use args::Args;

use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};

use inkanim::{
    anim::{InkAnimAnimationLibraryResource, OrphanInkAnimInterpolator, Target},
    widget::{inkWidgetLibraryResource, SiblingOrNested, WidgetTree},
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

fn into_table<'a>(value: DualResources) -> Vec<Table<'a>> {
    let DualResources {
        widget,
        anim,
        filter_by_type: _,
        show_path_names,
        filter_by_path,
    } = value;
    let mut tables: Vec<Table> = Vec::with_capacity(anim.sequences.len());
    let mut table: Table;
    let mut row: Row;
    for sequence in anim.sequences {
        table = Table::new();
        table.style = TableStyle::rounded();
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment(sequence.data.name.clone(), 2, Alignment::Center),
            TableCell::new_with_alignment("index", 1, Alignment::Center),
            TableCell::new_with_alignment("kind", 1, Alignment::Center),
            TableCell::new_with_alignment("time", 4, Alignment::Center),
            TableCell::new_with_alignment("direction", 1, Alignment::Center),
            TableCell::new_with_alignment("interpolation", 1, Alignment::Center),
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
            if let Some(ref filter) = filter_by_path {
                if infos
                    .as_ref()
                    .map(|x| !x.sibling_or_nested(filter))
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
                        fqcn.map(|x| x.join(" . "))
                            .unwrap_or_else(|| "".to_string()),
                        COLUMN_COUNT - 2,
                        Alignment::Left,
                    ),
                ]));
            } else {
                table.add_row(Row::new(vec![
                    TableCell::new(idx_definition),
                    TableCell::new(definition.handle_id),
                    TableCell::new_with_alignment(
                        infos
                            .clone()
                            .map(|x| {
                                x.iter()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<_>>()
                                    .join(" . ")
                            })
                            .unwrap_or_else(|| "".to_string()),
                        COLUMN_COUNT - 2,
                        Alignment::Left,
                    ),
                ]));
            }
            let kind = infos.clone().and_then(|x| {
                widget
                    .library_items
                    .get(0)
                    .expect("Root")
                    .package
                    .file
                    .data
                    .root_chunk
                    .get_widget_classname(&x)
            });

            for (idx_orphan, orphan) in interpolators.into_iter().enumerate() {
                if idx_orphan == 0 {
                    row = Row::new(vec![TableCell::new_with_col_span(
                        kind.as_ref()
                            .map(|x| x.to_string())
                            .unwrap_or_else(|| "".to_string())
                            .as_str(),
                        2,
                    )]);
                } else {
                    row = Row::new(vec![TableCell::new_with_col_span("", 2)]);
                }
                row.has_separator = idx_orphan == 0;
                row.cells.push(TableCell::new_with_alignment(
                    orphan.index,
                    1,
                    Alignment::Center,
                ));
                row.cells.push(TableCell::new_with_alignment(
                    orphan.interpolator.data.as_short_display(),
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
                    format!("{}", orphan.interpolator.data.transformation(),),
                    1,
                    Alignment::Left,
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
