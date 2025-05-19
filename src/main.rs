use clap::Parser;
use howmoji::howmoji_cli::cli::{self, Arguments};

fn main() {
    let args = Arguments::parse();

    if args.command {
        cli::run();
    } else {
        println!("👋 Use 'howmoji -c' to run the howmoji commit tool.👋");
    }
}
