use clap::Parser;
use cli::CLI;

mod args;
mod cli;
mod generate;
mod list;
mod read;
mod show;
mod tree;
mod whereis;
mod whois;

use generate::generate;
use list::list;
use read::read;
use show::show;
use tree::tree;
use whereis::whereis;
use whois::whois;

fn main() {
    let args = CLI::parse();
    let files = match args {
        CLI::List(list::Args { ref files, .. }) => files,
        CLI::Tree(tree::Args { ref files, .. }) => files,
        CLI::WhoIs(whois::Args { ref files, .. }) => files,
        CLI::WhereIs(whereis::Args { ref files, .. }) => files,
        CLI::Show(show::Args { ref files, .. }) => files,
        CLI::Generate(generate::Args { ref files, .. }) => files,
    };
    let (widget, anim) = read(files);
    match args {
        CLI::List(args) => list(args, widget.resource(), anim.resource()),
        CLI::Tree(args) => tree(args, widget),
        CLI::WhoIs(args) => whois(args, widget.resource(), anim.resource()),
        CLI::WhereIs(args) => whereis(args, widget.resource(), anim.resource()),
        CLI::Show(args) => show(args, widget.resource()),
        CLI::Generate(args) => generate(args, widget.resource()),
    };
}
