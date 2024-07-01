use chrono::{Date, Datelike, Local};
use std::fs::File;
use std::io::prelude::*;
use std::io::{empty, stdin, stdout, Write};
use std::path::Path;
use std::process::exit;

use crate::logic::*;
use crate::parser::*;

pub fn check_for_today() {
    let daily_records_list = parse_task_time_data();
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
            new_record_list.push(record);
        }

        if today_flag == true {
            println!("Task record alreay exists for today");
            println!("Do you want to overwrite your times for today?  Y/N");

            stdout().flush().unwrap();

            let mut change_task: String = String::new();
            stdin().read_line(&mut change_task).unwrap().to_string();
            change_task.pop();

            match change_task.as_str() {
                "y" | "yes" => {
                    println!("Adding new task times");
                    overwrite_tasks(&new_record_list)
                }
                "n" | "no" => {
                    println!("Not adding new task times. Exiting...");
                    exit(0)
                }
                _ => {
                    println!("Invalid option! Exiting!");
                    exit(0)
                }
            }
        }
    }
}

fn overwrite_tasks(new_record: &Vec<DailyRecord>) {
    let file = open_file(TASKTIMEPATH);
    let mut task_records: TaskRecords = json_to_struct_task_records(&file.as_str());

    task_records.daily_records = new_record.to_vec();

    let new_json = struct_task_records_to_json(task_records);

    _ = write_to_file(TASKTIMEPATH, new_json);
}

pub fn save_task_time(task_times: Vec<(String, f32)>, date: DateRecord) {
    _ = does_file_exist(TASKTIMEPATH);

    let new_record = DailyRecord::create(task_times, date);

    let _ = write_save_file(new_record);
}

pub fn write_save_file(new_record: DailyRecord) {
    let file = open_file(TASKTIMEPATH);
    let mut task_records: TaskRecords = json_to_struct_task_records(&file.as_str());

    task_records.daily_records.push(new_record);

    let new_json = struct_task_records_to_json(task_records);

    _ = write_to_file(TASKTIMEPATH, new_json);
}

pub fn write_to_file(path: &str, data: String) {
    let mut write_file = create_file(path);

    match write_file.write_all(data.as_bytes()) {
        Err(why) => panic!("couldn't deserialize from String to Tasks struct {}", why),
        Ok(()) => println!("Saved new task times"),
    };
}

pub fn write_task_file(tasks_data: Vec<(String, String)>) -> std::io::Result<()> {
    let mut file: Settings = json_to_struct_settings(open_file(SETTINGSPATH).as_str());

    for task_data in tasks_data {
        file.tasks.push(task_data);
    }

    let stringed_file: String = struct_settings_to_json(file);
    _ = write_to_file(SETTINGSPATH, stringed_file);

    Ok(())
}

pub fn does_file_exist(file_path: &str) {
    let path = Path::new(file_path);

    if !Path::exists(path) {
        if file_path == TASKTIMEPATH {
            let data_tr = TaskRecords::new();
            let new_json = struct_task_records_to_json(data_tr);
            _ = write_to_file(file_path, new_json);
        } else if file_path == SETTINGSPATH {
            let data_t = Settings::new();
            let new_json = struct_settings_to_json(data_t);
            _ = write_to_file(file_path, new_json);
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
