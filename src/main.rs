use crate::cmd_args::*;
use crate::logic::*;

mod cmd_args;
mod logic;
mod parser;

fn main() {
    let matches: clap::ArgMatches = subcmd_args().get_matches();

    app_title();

    match matches.subcommand() {
        Some(("task", sub_matches)) => task_cmd(sub_matches),
        Some(("review", sub_matches)) => review_cmd(sub_matches),
        _ => default_cmd(),
    };
}
