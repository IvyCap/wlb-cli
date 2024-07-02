use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};

use crate::shared::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub tasks: Vec<String>,
}

impl Settings {
    pub fn new() -> Settings {
        let new_record: Settings = Settings { tasks: vec![] };
        new_record
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TaskRecords {
    pub daily_records: Vec<DailyRecord>,
}

impl TaskRecords {
    pub fn new() -> TaskRecords {
        let new_record: TaskRecords = TaskRecords {
            daily_records: vec![],
        };
        new_record
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DailyRecord {
    pub date: DateRecord,
    pub task_time: Vec<(String, f32)>,
}

impl DailyRecord {
    pub fn create(task_times: Vec<(String, f32)>, date: DateRecord) -> DailyRecord {
        let new_record: DailyRecord = DailyRecord {
            date: date,
            task_time: task_times,
        };
        new_record
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub struct DateRecord {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl DateRecord {
    pub fn create_today() -> DateRecord {
        let date = Local::now();
        let new_date: DateRecord = DateRecord {
            year: date.year(),
            month: date.month(),
            day: date.day(),
        };
        new_date
    }
}

pub const SETTINGSPATH: &str = "./wlbSettings.json";
pub const TASKTIMEPATH: &str = "./taskTimeData.json";

pub fn parse_task_data() -> Vec<String> {
    _ = does_file_exist(SETTINGSPATH);

    let tasks_json: String = open_file(SETTINGSPATH);

    let v: Settings = json_to_struct_settings(tasks_json.as_str());

    let mut task_list: Vec<String> = vec![];

    for task in v.tasks {
        // println!("{:?}", task);
        task_list.push(task)
    }

    task_list
}

pub fn parse_task_time_data() -> Vec<DailyRecord> {
    _ = does_file_exist(TASKTIMEPATH);

    let task_time_data_json: String = open_file(TASKTIMEPATH);

    let v: TaskRecords = json_to_struct_task_records(task_time_data_json.as_str());

    let daily_records_list = v.daily_records;

    daily_records_list
}

pub fn json_to_struct_settings(tasks: &str) -> Settings {
    let v: Settings = match serde_json::from_str(tasks) {
        Err(why) => panic!(
            "couldn't deserialize from String to Settings struct {}",
            why
        ),
        Ok(file) => file,
    };

    v
}

pub fn json_to_struct_task_records(task_records: &str) -> TaskRecords {
    let v: TaskRecords = match serde_json::from_str(task_records) {
        Err(why) => panic!(
            "couldn't deserialize from String to TaskRecord struct: {}",
            why
        ),
        Ok(file) => file,
    };

    v
}

pub fn struct_settings_to_json(struct_t: Settings) -> String {
    let v: String = match serde_json::to_string(&struct_t) {
        Err(why) => panic!("couldn't serialize from Settingsstruct to String: {}", why),
        Ok(file) => file,
    };

    v
}

pub fn struct_task_records_to_json(struct_tr: TaskRecords) -> String {
    let v: String = match serde_json::to_string(&struct_tr) {
        Err(why) => panic!(
            "couldn't serialize from TaskRecord struct to String: {}",
            why
        ),
        Ok(file) => file,
    };

    v
}
