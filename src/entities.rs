use super::payroll_db::PayrollDB;

#[derive(Clone, PartialEq, Debug)]
pub enum PayType {
    Salaried,
}

#[derive(Clone)]
pub struct Employee {
    pub id: usize,
    pub pay_type: PayType,    
}

impl Employee {
    pub fn new(id: usize, pay_type: PayType) -> Employee {
        Employee {
            id,
            pay_type,
        }
    }
    pub fn execute(&mut self, db: &mut PayrollDB) {
        db.add_employee(self);
    }
}

