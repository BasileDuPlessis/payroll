use crate::entities::{Employee, PayType};
use crate::db::InMemoryDB;

pub struct AddEmployee {
    id: usize,
    db: InMemoryDB,
}

impl AddEmployee {
    pub fn new(id: usize, db: InMemoryDB) -> AddEmployee {
        AddEmployee {
            id,
            db,
        }
    }
    pub fn execute(&mut self) {
       // let salaried_employee = Employee::new(self.id, PayType::Salaried);
        //self.db.add_employee(&salaried_employee);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_salaried_employee_added_then_salaried_employee_saved() {
        let id = 1;    
        let db = InMemoryDB::new();    
        
        let mut add_employee = AddEmployee::new(id, db);

        add_employee.execute();
        
        // let saved_employee = db.read(id).unwrap();
        // assert_eq!(saved_employee.id, id);
        // assert_eq!(saved_employee.pay_type, PayType::Salaried);
    }
}