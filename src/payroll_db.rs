use std::collections::HashMap;
use super::entities::Employee;

pub struct PayrollDB {
    db: HashMap<usize, Employee>,
}

impl PayrollDB {
    pub fn new() -> PayrollDB {
        PayrollDB {
            db : HashMap::new()
        }
    }
    pub fn add_employee(&mut self, employee: &Employee) {
        self.db.insert(employee.id, employee.clone());  
    }
    pub fn read(&self, id: usize) -> Option<&Employee> {
        self.db.get(&id)
    }
}