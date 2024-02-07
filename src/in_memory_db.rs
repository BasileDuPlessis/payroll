use std::collections::HashMap;

trait CRUD {
    fn create(&mut self, entry: &str) -> usize;
    fn read(&self, id: usize) -> Option<String>;
    fn update(&mut self, id: usize, entry: &str);
    fn delete(&mut self, id: usize);
}



pub struct InMemoryDB {
    db: HashMap<usize, String>,
    last_index: usize,
}

impl InMemoryDB {
    pub fn new() -> InMemoryDB {
        InMemoryDB {
            db: HashMap::new(),
            last_index : 0,
        }
    }
}

impl CRUD for InMemoryDB {
    fn create(&mut self, entry: &str) -> usize {
        let id = self.last_index + 1;
        self.db.insert(id, entry.to_owned());
        self.last_index += 1;
        id
    }
    fn read(&self, id: usize) -> Option<String> {
        self.db.get(&id).cloned()
    }
    fn update(&mut self, id: usize, entry: &str) {
        let row = self.db.get_mut(&id).unwrap();
        *row = entry.to_owned();
    }
    fn delete(&mut self, id: usize) {
        self.db.remove(&id);
    }
}


mod MT {

    use once_cell::sync::Lazy;
    use std::sync::Mutex;

    static DB:Lazy<Mutex<super::HashMap<usize, String>>> = Lazy::new(|| {
        let mut m:super::HashMap<usize, String> = super::HashMap::new();
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
    
    impl super::CRUD for InMemoryDB {
        fn create(&mut self, entry: &str) -> usize {
            let mut last_index = LAST_INDEX.lock().unwrap();
            let id = *last_index + 1;
            DB.lock().unwrap().insert(id, entry.to_owned());
            *last_index += 1;        
            id
        }
        fn read(&self, id: usize) -> Option<String> {
            DB.lock().unwrap().get(&id).cloned()
        }
        fn update(&mut self, id: usize, entry: &str) {
            let mut db = DB.lock().unwrap();
            let row = db.get_mut(&id).unwrap();
            *row = entry.to_owned();
        }
        fn delete(&mut self, id: usize) {
            let mut db = DB.lock().unwrap();
            db.remove(&id);
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_entry() {
       let entry = "bob";
       let mut db = InMemoryDB::new();
       let id = db.create(entry);

        let e = db.read(id);

        assert_eq!(e.unwrap(), entry);
    }

    #[test]
    fn create_two_entry() {
        let entry1 = "bob";
        let entry2= "alice";
        let mut db = InMemoryDB::new();
        let id1 = db.create(entry1);
        let id2 = db.create(entry2);
        let e1 = db.read(id1);
        let e2 = db.read(id2);

        assert_eq!(e1.unwrap(), entry1);
        assert_eq!(e2.unwrap(), entry2);
    }

    #[test]
    fn update_entry() {
        let initial_entry = "bob";
        let updated_entry = "alice";
        let mut db = InMemoryDB::new();
        let id = db.create(initial_entry);

        db.update(id, updated_entry);

        let e = db.read(id);

        assert_eq!(updated_entry, e.unwrap());
    }

    #[test]
    fn delete_entry() {
        let initial_entry = "bob";
        let mut db = InMemoryDB::new();
        let id = db.create(initial_entry);
        
        db.delete(id);

        let e = db.read(id);

        assert_eq!(None, e);
    }

    #[test]
    fn create_entry_MT() {
       let entry = "bob";
       let mut db = MT::InMemoryDB::new();
       let id = db.create(entry);

        let e = db.read(id);

        assert_eq!(e.unwrap(), entry);
    }

    #[test]
    fn create_two_entry_MT() {
        let entry1 = "bob";
        let entry2= "alice";
        let mut db = MT::InMemoryDB::new();
        let id1 = db.create(entry1);
        let id2 = db.create(entry2);
        let e1 = db.read(id1);
        let e2 = db.read(id2);

        assert_eq!(e1.unwrap(), entry1);
        assert_eq!(e2.unwrap(), entry2);
    }

    #[test]
    fn update_entry_MT() {
        let initial_entry = "bob";
        let updated_entry = "alice";
        let mut db = MT::InMemoryDB::new();
        let id = db.create(initial_entry);

        db.update(id, updated_entry);

        let e = db.read(id);

        assert_eq!(updated_entry, e.unwrap());
    }

    #[test]
    fn delete_entry_MT() {
        let initial_entry = "bob";
        let mut db = MT::InMemoryDB::new();
        let id = db.create(initial_entry);
        
        db.delete(id);

        let e = db.read(id);

        assert_eq!(None, e);
    }

    #[test]
    fn multi_thread() {
        let entry1 = "bob";
        let mut db1 = MT::InMemoryDB::new();
        let id1 = db1.create(entry1);

        let entry2: &str = "alice";
        let mut db2 = MT::InMemoryDB::new();
        let id2 = db2.create(entry2);
        
        let db3 = MT::InMemoryDB::new();

        let e1 = db3.read(id1);
        let e2 = db3.read(id2);

        assert_eq!(entry1, e1.unwrap());
        assert_eq!(entry2, e2.unwrap());
    }

}