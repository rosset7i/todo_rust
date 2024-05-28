mod database;

use chrono::{DateTime, Utc};
use database::{Connection, Table};
fn main() {
    let cnn = Connection::from("todo.db");
}

struct List {
    name: String,
    itens: Vec<Item>,
}

impl Table for List {
    fn get_table_name() -> String {
        return String::from("List");
    }
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
