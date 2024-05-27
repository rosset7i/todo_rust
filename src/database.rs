use sqlite::open;

use crate::{Item, List};

pub trait Table {
    fn get_table_name() -> String;
}

struct Connection {
    connection_name: String,
}

impl Connection {
    fn from(connection_string: &str) -> Connection {
        return Connection {
            connection_name: String::from(connection_string),
        };
    }

    fn create<T: Table>(&self) {
        let cnn = open(&self.connection_name).unwrap();

        //TODO: FIX THIS SHIH
        cnn.execute(format!(
            "INSERT INTO {table_name} VALUES",
            table_name = T::get_table_name()
        ))
        .unwrap();
    }
}

pub fn migrate_database() {
    let cnn = get_connection().unwrap();

    cnn.execute(format!(
        "CREATE TABLE IF NOT EXISTS {table_name} (name TEXT)",
        table_name = List::get_table_name()
    ))
    .unwrap();

    cnn.execute(
            format!("CREATE TABLE IF NOT EXISTS {table_name} (name TEXT, description TEXT, is_finished INTEGER,finished_time TEXT, list_id INTEGER)", table_name = Item::get_table_name()),
    )
    .unwrap();
}

pub fn add(list: List) {
    let cnn = get_connection().unwrap();

    cnn.execute("Test");
}

pub fn get_connection() -> Result<sqlite::Connection, sqlite::Error> {
    return open("todo.db");
}
