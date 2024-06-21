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

fn ask_hours(task: String) -> f32 {
    print!("{}: ", task);
    stdout().flush().unwrap();

    let mut task_time = String::new();
    stdin().read_line(&mut task_time).unwrap();
    let perc_time: f32;
    task_time.pop();
    if task_time.is_empty() {
        perc_time = 0.0;
    } else {
        perc_time = task_time.parse().unwrap();
    }
    perc_time
}

fn percent_calc(task_time: f32, num_days: f32) -> f32 {
    let perc_time: f32;
    perc_time = (task_time / (HOURSDAY * num_days)) * 100.00;
    perc_time
}

pub fn get_times(tasks: Vec<(String, String)>) -> Vec<(String, f32)> {
    let mut task_time = vec![];
    let mut total_time: f32 = 0.0;
    println!("Enter in how many hours have you spent on these tasks in the last 24 hours.");
    println!("(Exp: 3, 2.50, 0.25)");
    println!("");
    for task in tasks {
        let time = ask_hours(task.1);
        total_time += &time;
        let title_time = (task.0, time);
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
            "Please reenter the hours and ensure they do not exceed 24 hours"
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

pub fn print_tasks_percent(titles_times: &Vec<(String, f32)>) {
    println!("");
    println!("{}", "Task Percentages for the Day".cyan());

    for time in titles_times {
        let per_time = percent_calc(time.1, 1.0);
        println!(
            "{}: {:.2}{}",
            time.0.bright_blue(),
            per_time.to_string().on_yellow().black(),
            "%".on_yellow().black()
        );
    }
}

pub fn print_tasks_list(task_list: &Vec<(String, String)>) {
    println!("");
    println!("{}", "Task List".bold().cyan());

    for task in task_list {
        println!("{}: {}", task.0.bright_blue(), task.1.blue());
    }
    println!("");
}
