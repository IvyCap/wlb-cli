use crate::cmd_args::*;
use crate::logic::*;
use crate::shared::*;

mod cmd_args;
mod logic;
mod parser;
mod shared;

fn main() {
    let matches: clap::ArgMatches = subcmd_args().get_matches();

    app_title();

    is_task_list_empty();

    match matches.subcommand() {
        Some(("task", sub_matches)) => task_cmd(sub_matches),
        Some(("review", sub_matches)) => review_cmd(sub_matches),
        _ => default_cmd(),
    };
}
