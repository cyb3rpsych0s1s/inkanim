mod args;

pub(crate) use args::Args;
use inkanim_types::{
    anim::InkAnimAnimationLibraryResource,
    widget::{WidgetTree, inkWidgetLibraryResource},
};
use term_table::{
    Table, TableStyle,
    row::Row,
    table_cell::{Alignment, TableCell},
};

fn json(_: &[&str], sequences: &[usize]) {
    let json = serde_json::to_string_pretty(&sequences).unwrap();
    println!("{json}");
}
fn table(names: &[&str], indexes: &[usize]) {
    let mut table = Table::new();
    table.style = TableStyle::rounded();
    table.add_row(Row::new(
        indexes
            .iter()
            .map(|x| {
                TableCell::builder(x)
                    .col_span(1)
                    .alignment(Alignment::Center)
                    .build()
            })
            .collect::<Vec<_>>(),
    ));
    table.add_row(Row::new(
        names
            .iter()
            .map(|x| {
                TableCell::builder(x)
                    .col_span(1)
                    .alignment(Alignment::Center)
                    .build()
            })
            .collect::<Vec<_>>(),
    ));
    println!("{}", table.render());
}

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
        let names = names.iter().map(|x| x.as_str()).collect::<Vec<_>>();
        match args.mode.output {
            crate::args::Output::Table => {
                table(names.as_slice(), indexes.as_slice());
            }
            crate::args::Output::Json => {
                json(names.as_slice(), indexes.as_slice());
            }
            crate::args::Output::Reds => todo!(),
        };
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
