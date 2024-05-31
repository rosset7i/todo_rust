use sqlite::open;

use crate::Item;

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

    fn get_connection(&self) -> sqlite::Connection {
        return open(&self.connection_name).unwrap();
    }

    pub fn query(&self, sql: String) -> Result<(), sqlite::Error>{
        let cnn = self.get_connection();
        return cnn.iterate(sql, |x| {
            for &(name, value) in x.iter() {
                println!("{} = {}", name, value.unwrap());
            }
            true
        });
    }

    pub fn create(&self, sql: String) {
        let cnn = self.get_connection();

        cnn.execute(sql).unwrap();
    }

    pub fn remove_by_id<T: Table>(&self, id: u32) {
        let cnn = self.get_connection();

        cnn.execute(format!(
            "DELETE FROM {table_name} WHERE id = {id}",
            table_name = T::get_table_name(),
            id = id
        ))
        .unwrap();
    }

    fn migrate_database(&self) {
        let cnn = self.get_connection();

        cnn.execute(
            format!("CREATE TABLE IF NOT EXISTS {table_name} (id INTEGER PRIMARY KEY, name TEXT, description TEXT, is_finished INTEGER,finished_time TEXT)", table_name = Item::get_table_name()),
    
        )
        .unwrap();
    }
}
