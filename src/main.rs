mod database;

use chrono::{DateTime, Utc};
use database::{Connection, Table};
use std::io::stdin;

fn main() {
    let cnn = Connection::from("todo.db");

    let mut quit = false;

    while !quit {
        let buffer = get_input();

        match buffer.as_str().trim() {
            "a" => add_to_do(&cnn),
            "r" => remove_to_do(&cnn),
            "m" => mark_as_done(&cnn),
            "l" => see_to_do(&cnn),
            "q" => quit = true,
            _ => println!("{}", buffer.as_str()),
        }
    }
}

fn see_to_do(cnn: &Connection) {
    cnn.query("SELECT * FROM ITEM".to_string()).unwrap();
}

fn mark_as_done(cnn: &Connection) {
    let input: u32 = get_input().parse().unwrap();

    cnn.create(format!(
        "UPDATE ITEM SET is_finished = 1, finished_time = '{date}'
        WHERE id = {id}",
        id = input,
        date = Utc::now(),
    ));
}

fn remove_to_do(cnn: &Connection) {
    println!("Which item do you want do delete?: ");

    let input: u32 = get_input().parse().unwrap();
    cnn.remove_by_id::<Item>(input);
}

fn get_input() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    return buffer.trim().to_string();
}

fn add_to_do(cnn: &Connection) {
    println!("Please inform a name for the task: ");
    let name = get_input();
    println!("Please inform a description: ");
    let description = get_input();

    cnn.create(format!(
        "INSERT INTO {table} VALUES(NULL,'{name}','{description}',{is_finished},'{finished_time}')",
        table = Item::get_table_name(),
        name = name,
        description = description,
        is_finished = false,
        finished_time = String::new()
    ));
}

struct Item {
    name: String,
    description: String,
    is_finished: bool,
    finished_time: DateTime<Utc>,
}

impl Table for Item {
    fn get_table_name() -> String {
        return String::from("Item");
    }
}
