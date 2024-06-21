use clap::{arg, command, ArgMatches, Command};

use crate::logic::*;
use crate::parser::*;

// //Todo: Add edit arg to edit task_data json file with text editor

// //Todo: Add args to get task percetages for the week, month, year, ytd from historic task_data json file

// //Todo: Add args to add task and review task lists

pub fn subcmd_args() -> Command {
    command!()
        .about("Track and calculate work life balance calculator")
        .subcommand(
            Command::new("task")
                .about("Modify or show task lists")
                .arg(arg!(add: -a --add [ADD]"Add task to task list"))
                .arg(arg!(edit: -e --edit [EDIT] "Edit task to task list"))
                .arg(arg!(show: -s --show [SHOW] "Display task list"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("review")
                .about("Review time spent on tasks")
                .arg(arg!(-w --week [WEEK] "Review task data over the current week"))
                .arg_required_else_help(true),
        )
}

pub fn default_cmd() {
    let task_data = parse_task_data();

    let task_times: Vec<(String, f32)> = get_times(task_data);

    print_tasks_percent(&task_times);

    save_task_time(task_times);
}

pub fn task_cmd(matches: &ArgMatches) {
    let task_data = parse_task_data();
    if let Some(add) = matches.get_many::<String>("add") {
        println!("Adding Tasks");
    } else if let Some(edit) = matches.get_many::<String>("edit") {
        println!("Editing Tasks");
    } else if let Some(show) = matches.get_many::<String>("show") {
        print_tasks_list(&task_data)
    }
}

pub fn review_cmd(matches: &ArgMatches) {
    let task_time_data = parse_task_time_data();
    if let Some(week) = matches.get_many::<String>("week") {
        print_tasks_percent(&task_time_data)
    }
}
