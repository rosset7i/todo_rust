use sqlite::open;

use crate::{Item, List};

pub trait Table {
    fn get_table_name() -> String;
}

pub struct Connection {
    connection_name: String,
}

impl Connection {
    pub fn from(connection_string: &str) -> Connection {
        let cnn = Connection {
            connection_name: connection_string.to_string()
        };

        cnn.migrate_database();
        return cnn;
    }

    fn get_connection(&self) -> Result<sqlite::Connection, sqlite::Error> {
        return open(&self.connection_name);
    }

    pub fn sql(&self, sql: &str) -> Result<(), sqlite::Error>{
        let cnn = self.get_connection().unwrap();
        return cnn.execute(sql);
    }

    pub fn remove_by_id<T: Table>(&self, id: u32) {
        let cnn = self.get_connection().unwrap();

        cnn.execute(format!(
            "DELETE FROM {table_name} x WHERE x.Id = {id}",
            table_name = T::get_table_name(),
            id = id
        ))
        .unwrap();
    }

    fn migrate_database(&self) {
        let cnn = self.get_connection().unwrap();

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
}
