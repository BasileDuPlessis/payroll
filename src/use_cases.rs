use crate::entities::{Employee, PayType};
use crate::payroll_db::PayrollDB;

pub fn add_salaried_employee(id: usize, db: &mut PayrollDB) {
    let mut salaried_employee = Employee::new(id, PayType::Salaried);
    salaried_employee.execute(db);
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn when_salaried_employee_added_then_salaried_employee_saved() {
        let id = 1;
        let mut db = PayrollDB::new();
        
        add_salaried_employee(id, &mut db);
        
        let saved_employee = db.read(id).unwrap();

        assert_eq!(saved_employee.id, id);
        assert_eq!(saved_employee.pay_type, PayType::Salaried);

    }
}