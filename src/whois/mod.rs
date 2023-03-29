mod args;

pub(crate) use args::Args;
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};

use inkanim::{inkWidgetLibraryResource, InkAnimAnimationLibraryResource, WidgetTree};

pub(crate) fn whois(
    args: Args,
    widget: inkWidgetLibraryResource,
    anim: InkAnimAnimationLibraryResource,
) {
    let indexes = args.path.path;
    let depth = indexes.len();
    let sequences: Vec<&str> = anim.sequences.iter().map(|x| x.name()).collect();
    let found = widget.get_path_names(&indexes);

    if let Some(names) = found {
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
            indexes
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" . "),
            sequences.join(", "),
        );
    }
}
