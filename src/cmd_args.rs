use chrono::{Datelike, Local};
use clap::{arg, command, ArgMatches, Command};
use colored::*;

use crate::logic::*;
use crate::parser::*;
use crate::shared::*;

// //Todo: Add args to get task percetages for the week, month, year, ytd from historic task_data json file

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
                .arg(arg!(-t --today [TODAY] "Review todays task data"))
                .arg(arg!(-w --week [WEEK] "Review task data over the current week"))
                .arg(arg!(-m --month [MONTH] "Review task data over the current month"))
                .arg(arg!(-y --ytd [YTD] "Review task data over the current year"))
                .arg_required_else_help(true),
        )
}

// pub fn default_cmd() {}

pub fn default_cmd(settings_file: &Settings) {
    let task_data = parse_task_data();

    check_for_today(settings_file);

    let task_times: Vec<(String, f32)> = get_times(task_data);

    print_tasks_percent(&task_times, 1.0);

    let date = DateRecord::create_today();

    save_task_time(task_times, date, settings_file);
}

pub fn task_cmd(matches: &ArgMatches) {
    let task_data = parse_task_data();
    if let Some(_add) = matches.get_many::<String>("add") {
        add_task_to_list();
    } else if let Some(_edit) = matches.get_many::<String>("edit") {
        edit_tasklist();
    } else if let Some(_show) = matches.get_many::<String>("show") {
        print_tasks_list(&task_data)
    }
}

pub fn review_cmd(matches: &ArgMatches, settings_file: &Settings) {
    let now = Local::now();
    let daily_records_list = parse_task_time_data(settings_file);
    if let Some(_today) = matches.get_many::<String>("today") {
        for record in daily_records_list {
            if record.date.year == now.year()
                && record.date.month == now.month()
                && record.date.day == now.day()
            {
                let day_task_time = record.task_time;
                println!("");
                println!(
                    "{} {}-{}-{}",
                    "Task Percentages for today:".cyan(),
                    record.date.year.to_string().cyan(),
                    record.date.month.to_string().cyan(),
                    record.date.day.to_string().cyan()
                );
                print_tasks_percent(&day_task_time, 1.0)
            }
        }
    } else if let Some(_month) = matches.get_many::<String>("month") {
        let mut combined_record: Vec<Vec<(String, f32)>> = vec![];
        let mut days_of_tasks: f32 = 0.0;
        for record in daily_records_list {
            if record.date.month == now.month() {
                days_of_tasks += 1.0;
                combined_record.push(record.task_time)
            }
        }
        let day_task_time = combined_task_times(combined_record);
        println!("");
        println!(
            "{} {}",
            "Task Percentages for the month of:".cyan(),
            now.format("%B").to_string().cyan()
        );
        println!("Number of days tasks where logged: {}", &days_of_tasks);
        print_tasks_percent(&day_task_time, days_of_tasks)
    } else if let Some(_ytd) = matches.get_many::<String>("ytd") {
        let mut combined_record: Vec<Vec<(String, f32)>> = vec![];
        let mut days_of_tasks: f32 = 0.0;
        for record in daily_records_list {
            if record.date.year == now.year() {
                days_of_tasks += 1.0;
                combined_record.push(record.task_time)
            }
        }
        let day_task_time = combined_task_times(combined_record);
        println!("");
        println!(
            "{} {}",
            "Task Percentages for this year:".cyan(),
            now.year().to_string().cyan()
        );
        println!("Number of days tasks where logged: {}", &days_of_tasks);
        print_tasks_percent(&day_task_time, days_of_tasks)
    }
}
