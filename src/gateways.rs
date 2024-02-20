use crate::db::InMemoryDBCrud;
use crate::entities::Employee;
use crate::db::{InMemoryDB, DBError};
use serde_json::error::Error as JSONError;

#[derive(Debug)]
pub enum GatewayError {
    JSON(JSONError),
    DB(DBError),
}

impl From<JSONError> for GatewayError {
    fn from(e: JSONError) -> Self {
        Self::JSON(e)
    }
}

impl From<DBError> for GatewayError {
    fn from(e: DBError) -> Self {
        Self::DB(e)
    }
}

pub trait Gateway {
    fn add_employee(&mut self, employee: &Employee) -> Result<usize, GatewayError>;
    fn read_employee(&self, id: usize) -> Result<Employee, GatewayError>;
}


pub struct InMemoryDbGateway {
    db: InMemoryDB,
}

impl InMemoryDbGateway {
    pub fn new() -> InMemoryDbGateway {
        InMemoryDbGateway {
            db: InMemoryDB::new(),
        }
    }
}

impl Gateway for InMemoryDbGateway {
    fn add_employee(&mut self, employee: &Employee) -> Result<usize, GatewayError> {        
        
        let entry = serde_json::to_string(employee)?;

        let id = self.db.create(&entry)?;
        
        Ok(id)
    }

    fn read_employee(&self, id: usize) -> Result<Employee, GatewayError> {   

        let entry = self.db.read(id)?;

        let employee: Employee = serde_json::from_str(&entry)?;

        Ok(employee)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_employee_added_can_read_employee() {
        let mut gateway = InMemoryDbGateway::new();
        let employee = Employee::new("bob");
        let id = gateway.add_employee(&employee).unwrap();

        let e = gateway.read_employee(id).unwrap();

        assert_eq!(e.name, employee.name);
    }

}