use chrono::{Date, Datelike, Local};
use colored::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::{empty, stdin, stdout, Write};
use std::path::Path;
use std::process::exit;

use crate::logic::*;
use crate::parser::*;

pub fn overwrite_tasks(new_record: &Vec<DailyRecord>) {
    let file = open_file(TASKTIMEPATH);
    let mut task_records: TaskRecords = json_to_struct_task_records(&file.as_str());

    task_records.daily_records = new_record.to_vec();

    let new_json = struct_task_records_to_json(task_records);

    write_to_file(TASKTIMEPATH, new_json);
}

pub fn save_task_time(task_times: Vec<(String, f32)>, date: DateRecord) {
    does_file_exist(TASKTIMEPATH);

    let new_record = DailyRecord::create(task_times, date);

    let _ = write_save_file(new_record);
}

pub fn write_save_file(new_record: DailyRecord) {
    let file = open_file(TASKTIMEPATH);
    let mut task_records: TaskRecords = json_to_struct_task_records(&file.as_str());

    task_records.daily_records.push(new_record);

    let new_json = struct_task_records_to_json(task_records);

    write_to_file(TASKTIMEPATH, new_json);
}

pub fn write_to_file(path: &str, data: String) {
    let mut write_file = create_file(path);

    match write_file.write_all(data.as_bytes()) {
        Err(why) => panic!("couldn't deserialize from String to Tasks struct {}", why),
        Ok(()) => println!("Saved new task times"),
    };
}

pub fn write_task_file(tasks_data: Vec<String>) {
    let mut file: Settings = json_to_struct_settings(open_file(SETTINGSPATH).as_str());

    for task_data in tasks_data {
        file.tasks.push(task_data);
    }

    let stringed_file: String = struct_settings_to_json(file);
    write_to_file(SETTINGSPATH, stringed_file);
}

pub fn does_file_exist(file_path: &str) {
    let path = Path::new(file_path);

    if !Path::exists(path) {
        if file_path == TASKTIMEPATH {
            let data_tr = TaskRecords::new();
            let new_json = struct_task_records_to_json(data_tr);
            write_to_file(file_path, new_json);
        } else if file_path == SETTINGSPATH {
            let data_t = Settings::new();
            let new_json = struct_settings_to_json(data_t);
            write_to_file(file_path, new_json);
            println!("Task list does not exist. Create new task list");
            add_task_to_list();
        }
    }
}

pub fn create_file(file_path: &str) -> File {
    let file = match File::create(&file_path) {
        Err(why) => panic!("couldn't create {}: {}", &file_path, why),
        Ok(file) => file,
    };

    file
}

pub fn open_file(path: &str) -> String {
    let file = File::open(path);

    let mut file = match file {
        Ok(f) => f,
        Err(error) => {
            panic!("Error: {:?}", error)
        }
    };

    let mut contents = String::new();
    let contents = match file.read_to_string(&mut contents) {
        Ok(_) => contents,
        Err(error) => {
            panic!("Error: {:?}", error)
        }
    };

    contents
}

pub fn print_tasks_percent(titles_times: &Vec<(String, f32)>, num_days: f32) {
    for time in titles_times {
        let per_time = percent_calc(time.1, num_days);
        println!("");
        println!(
            "{}:{}{}{:.2}{}",
            time.0.blue(),
            time.1.to_string().yellow(),
            "hrs/".yellow(),
            per_time.to_string().on_yellow().black(),
            "%".on_yellow().black()
        );
        println!("");
    }
}

pub fn print_tasks_list(task_list: &Vec<String>) {
    println!("");
    println!("{}", "Task List".bold().cyan());

    for task in task_list {
        println!("{}", task.bright_blue());
    }
    println!("");
}

pub fn add_task_to_list() {
    println!("");
    println!("Enter in task names to add to task list. One task per line.");
    println!("Leave line blank and press enter to save new tasks to task list");

    let mut task_data = parse_task_data();
    let mut enter_task_flag = true;

    while enter_task_flag == true {
        let mut new_task = String::new();
        print!("New task: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut new_task).unwrap().to_string();
        new_task.pop();
        match new_task.as_str() {
            "" => {
                verify_task_list(&task_data);
                enter_task_flag = false;
            }
            _ => task_data.push(new_task),
        }
    }
}

fn verify_task_list(task_list: &Vec<String>) {
    print_tasks_list(task_list);
    let mut change_task: String = String::new();
    println!("Do you want to save changes to the task list?");
    print!("Y/N: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut change_task).unwrap().to_string();
    change_task.pop();

    match change_task.as_str() {
        "y" | "yes" => {
            println!("Adding new task times");
            write_task_file(task_list.clone())
        }
        "n" | "no" => {
            println!("Not saving changes. Exiting...");
            exit(0)
        }
        _ => {
            println!("Invalid option! Exiting!");
            exit(0)
        }
    }
}
