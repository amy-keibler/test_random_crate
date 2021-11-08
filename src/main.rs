use structopt::StructOpt;

mod cli;
use crate::cli::Cli;

mod config;

fn main() {
    let cli = Cli::from_args();
    println!("Cli:\n{:#?}", cli);
}
