use redb::Database;
use uuid::Uuid;

use super::super::data::model::{File, Metadata};

pub struct Files<'a> {
    db: &'a Database
}

impl<'a> Files<'a> {
    fn new(db: &'a Database) -> Self {
        Self { db }
    }
}

impl<'a> super::super::Files for Files<'a> {
    fn find_all() -> Vec<Metadata> {
        
        todo!()
    }

    fn content(id: Uuid) -> String {
        todo!()
    }

    fn add(file: File) -> () {
        todo!()
    }

    fn update_metadata(metadata: Metadata) -> () {
        todo!()
    }

    fn update_content(id: Uuid, content: String) -> () {
        todo!()
    }

    fn delete(id: Uuid) -> () {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use redb::{Builder, TableDefinition, backends::InMemoryBackend};

    use super::*;

    #[test]
    fn test_find_all() {
        let db = test_db();

        // let tbl: TableDefinition<'_, Uuid, Metadata> = TableDefinition::new("test-table");

        // let tx = db.begin_write().expect("Expected write transaction");
        // let open_tbl = tx.open_table(tbl).expect("Expected table to open");
    }

    fn test_db() -> Database {
        let backend = InMemoryBackend::new();
        Builder::new().create_with_backend(backend).expect("Expected in-memory database to be created")
    }
}