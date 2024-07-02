use crate::parser::*;
use crate::shared::*;

use chrono::{Datelike, Local};
use colored::*;
use std::io::{stdin, stdout, Write};
use std::process::exit;

const HOURSDAY: f32 = 24.0;

pub fn app_title() {
    println!("");
    println!("");
    println!("{}", "Work Life Balance Calculator".bold().cyan());
    println!("");
}

pub fn ask_hours(task: String) -> f32 {
    print!("{}: ", task);
    stdout().flush().unwrap();

    let mut task_time = String::new();
    stdin().read_line(&mut task_time).unwrap().to_string();
    let perc_time: f32;
    task_time.pop();
    if task_time.is_empty() {
        perc_time = 0.0;
    } else {
        perc_time = task_time.parse().unwrap();
    }
    perc_time
}

pub fn percent_calc(task_time: f32, num_days: f32) -> f32 {
    let perc_time: f32;
    perc_time = (task_time / (HOURSDAY * num_days)) * 100.00;
    perc_time
}

pub fn get_times(tasks: Vec<String>) -> Vec<(String, f32)> {
    let mut task_time = vec![];
    let mut total_time: f32 = 0.0;
    println!("Enter in how many hours have you spent on these tasks in the last 24 hours.");
    println!("(Exp: 3, 2.50, 0.25)");
    println!("");
    for task in tasks {
        let time = ask_hours(task.clone());
        total_time += &time;
        let title_time = (task, time);
        task_time.push(title_time);
    }

    if total_time > 24.0 {
        println!("{}", "WARNING:".on_red().bold().black());
        println!(
            "{}",
            "Hours entered equal more than 24 hours."
                .on_red()
                .bold()
                .black()
        );
        println!(
            "{}",
            "Please re-enter the hours and ensure they do not exceed 24 hours"
                .on_red()
                .bold()
                .black()
        );
        exit(0)
    } else if total_time < 24.0 {
        println!("");
        println!("NOTICE:");
        println!("Total hours are less than 24 hours.");
        let unused_time = HOURSDAY - total_time;
        let unused_name_time = ("Undocumented Time".to_string(), unused_time);
        task_time.push(unused_name_time);
    }
    task_time
}

pub fn combined_task_times(combined_recods: Vec<Vec<(String, f32)>>) -> Vec<(String, f32)> {
    let mut new_combined_list: Vec<(String, f32)> = pre_populate_task_list();

    for day_tasks in combined_recods {
        for task in day_tasks {
            let mut in_list_flag: bool = false;
            let mut list_index = 0;
            for new_task in new_combined_list.clone() {
                if new_task.0.to_lowercase() == task.0.to_lowercase() {
                    let new_time = (new_task.0.clone(), new_task.1 + task.1);
                    new_combined_list.remove(list_index);
                    new_combined_list.push(new_time);
                    in_list_flag = true;
                }
                list_index += 1;
            }
            if in_list_flag == false {
                new_combined_list.push(task)
            }
        }
    }
    new_combined_list
}

pub fn pre_populate_task_list() -> Vec<(String, f32)> {
    let task_list = parse_task_data();
    let mut populated_list: Vec<(String, f32)> = vec![];

    for task in task_list {
        let new_tup = (task, 0.0);
        populated_list.push(new_tup);
    }

    populated_list
}

pub fn check_for_today(settings_file: &Settings) {
    let daily_records_list = parse_task_time_data(settings_file);
    let mut today_flag = false;
    let now = Local::now();
    let mut new_record_list: Vec<DailyRecord> = vec![];

    for record in daily_records_list {
        if record.date.year == now.year()
            && record.date.month == now.month()
            && record.date.day == now.day()
        {
            today_flag = true
        } else {
            new_record_list.push(record.clone());
        }

        if today_flag == true {
            let task_time = record.task_time;
            println!("Task record already exists for today");
            println!("");
            print_tasks_percent(&task_time, 1.0);
            println!("Do you want to overwrite your times for today?");
            print!("Y/N: ");
            stdout().flush().unwrap();

            let mut change_task: String = String::new();
            stdin().read_line(&mut change_task).unwrap().to_string();
            change_task.pop();

            match change_task.as_str() {
                "y" | "yes" => {
                    println!("Adding new task times");
                    overwrite_tasks(&new_record_list, settings_file)
                }
                "n" | "no" => {
                    println!("Not adding new task times. Exiting...");
                    println!("");
                    exit(0)
                }
                _ => {
                    println!("Invalid option! Exiting!");
                    println!("");
                    exit(0)
                }
            }
        }
    }
}
