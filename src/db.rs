use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum DBError {
    Create,
    Read,
}

pub trait InMemoryDBCrud {
    fn create(&mut self, entry: &str) -> Result<usize, DBError>;
    fn read(&self, id: usize) -> Result<String, DBError>;
    fn update(&mut self, id: usize, entry: &str);
    fn delete(&mut self, id: usize);
}


use once_cell::sync::Lazy;
use std::sync::Mutex;

static DB:Lazy<Mutex<HashMap<usize, String>>> = Lazy::new(|| {
    let mut m:HashMap<usize, String> = HashMap::new();
    Mutex::new(m)
});
static LAST_INDEX:Lazy<Mutex<usize>> = Lazy::new(|| {
    let mut m:usize = 0;
    Mutex::new(m)
});

pub struct InMemoryDB {}

impl InMemoryDB {
    pub fn new() -> InMemoryDB {
        InMemoryDB {}
    }
}

impl InMemoryDBCrud for InMemoryDB {
    fn create(&mut self, entry: &str) -> Result<usize, DBError> {
        if let Ok(mut last_index) = LAST_INDEX.lock() {                
            if let Ok(mut db) = DB.lock() {
                let id = *last_index + 1;
                db.insert(id, entry.to_owned());
                *last_index += 1;
                return Ok(id);
            }
        }
        Err(DBError::Create)
    }
    fn read(&self, id: usize) -> Result<String, DBError> {
        if let Ok(mut db) = DB.lock() {
            if let Some(entity) = db.get(&id) {
                return Ok(entity.to_string());
            }            
        }
        Err(DBError::Read)
    }
    fn update(&mut self, id: usize, entry: &str) {
        if let Ok(mut db) = DB.lock() {
            if let Some(row) = db.get_mut(&id) {
                *row = entry.to_owned();
            }
        }
    }
    fn delete(&mut self, id: usize) {
        if let Ok(mut db) = DB.lock() {
            db.remove(&id);
        }            
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_entry_static() {
       let entry = "bob";
       let mut db = InMemoryDB::new();
       let id = db.create(entry).unwrap();

        let e = db.read(id).unwrap();

        assert_eq!(e, entry);
    }

    #[test]
    fn create_two_entry_static() {
        let entry1 = "bob";
        let entry2= "alice";
        let mut db = InMemoryDB::new();
        let id1 = db.create(entry1).unwrap();
        let id2 = db.create(entry2).unwrap();
        let e1 = db.read(id1);
        let e2 = db.read(id2);

        assert_eq!(e1.unwrap(), entry1);
        assert_eq!(e2.unwrap(), entry2);
    }

    #[test]
    fn update_entry_static() {
        let initial_entry = "bob";
        let updated_entry = "alice";
        let mut db = InMemoryDB::new();
        let id = db.create(initial_entry).unwrap();

        db.update(id, updated_entry);

        let e = db.read(id);

        assert_eq!(updated_entry, e.unwrap());
    }

    #[test]
    fn delete_entry_static() {
        let initial_entry = "bob";
        let mut db = InMemoryDB::new();
        let id = db.create(initial_entry).unwrap();
        
        db.delete(id);

        let e = db.read(id);

        assert_eq!(DBError::Read, e.unwrap_err());
    }

    #[test]
    fn multi_instance() {
        let entry1 = "bob";
        let mut db1 = InMemoryDB::new();
        let id1 = db1.create(entry1).unwrap();

        let entry2: &str = "alice";
        let mut db2 = InMemoryDB::new();
        let id2 = db2.create(entry2).unwrap();
        
        let db3 = InMemoryDB::new();

        let e1 = db3.read(id1);
        let e2 = db3.read(id2);

        assert_eq!(entry1, e1.unwrap());
        assert_eq!(entry2, e2.unwrap());
    }

}