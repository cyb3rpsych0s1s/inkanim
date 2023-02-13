mod args;
pub(crate) use args::Args;
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};

use crate::ink::{inkWidgetLibraryResource, InkAnimAnimationLibraryResource, WidgetTree};

pub(crate) fn whereis(
    args: Args,
    widget: inkWidgetLibraryResource,
    anim: InkAnimAnimationLibraryResource,
) {
    // let names = args.names.path;
    // if names.len() == 0 {
    //     panic!("please specify widget path names");
    // }
    // let indexes = widget.get_path_indexes(&names.iter().map(|x| x.as_str()).collect::<Vec<_>>());
    // println!("{:#?}", indexes);
    let names = args.names.path;
    let depth = names.len();
    let sequences: Vec<&str> = anim.sequences.iter().map(|x| x.name()).collect();
    let found = widget.get_path_indexes(&names.iter().map(|x| x.as_str()).collect::<Vec<_>>());

    if let Some(indexes) = found {
        assert_eq!(depth, names.len());

        let mut table = Table::new();
        table.style = TableStyle::rounded();
        table.add_row(Row::new(
            indexes
                .iter()
                .map(|x| TableCell::new_with_alignment(x, 1, Alignment::Center))
                .collect::<Vec<_>>(),
        ));
        table.add_row(Row::new(
            names
                .iter()
                .map(|x| TableCell::new_with_alignment(x, 1, Alignment::Center))
                .collect::<Vec<_>>(),
        ));
        println!("{}", table.render());
    } else {
        println!(
            "couldn't find\n{}\nin sequence(s): {}",
            names
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" . "),
            sequences.join(", "),
        );
    }
}
