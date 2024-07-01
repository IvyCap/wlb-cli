use chrono::{Date, Datelike, Local};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::{self, Path};

use crate::logic::*;
use crate::parser::*;

pub fn check_for_today() {}

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

pub fn write_task_data(tasks_data: Vec<(String, String)>) -> std::io::Result<()> {
    let mut file: Tasks = json_to_struct_tasks(open_file(TASKPATH).as_str());

    for task_data in tasks_data {
        file.tasks.push(task_data);
    }

    // f.write_all(b"test text")?;

    Ok(())
}

pub fn does_file_exist(file_path: &str) {
    let path = Path::new(file_path);

    if !Path::exists(path) {
        if file_path == TASKTIMEPATH {
            let data_tr = TaskRecords::new();
            let new_json = struct_task_records_to_json(data_tr);
            _ = write_to_file(file_path, new_json);
        } else if file_path == TASKPATH {
            let data_t = Tasks::new();
            let new_json = struct_tasks_to_json(data_t);
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

    // dbg!("File: {:?}", &file);

    let mut contents = String::new();
    let contents = match file.read_to_string(&mut contents) {
        Ok(_) => contents,
        Err(error) => {
            panic!("Error: {:?}", error)
        }
    };

    // dbg!("Contents: {}", &contents);

    contents
}
