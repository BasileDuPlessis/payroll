mod entities;
mod in_memory_db;
mod use_cases;
//mod use_cases_builder;


fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use self::use_cases::AddEmployee;

    use super::*;

    #[test]
    fn when_salaried_employee_added_then_salaried_employee_saved() {
        //let add_employee = AddEmployee::new(id, db);
        //Add Employee new(name, db) => builder
        //Add Salary Type
        //Execute

        //Read DB
    }

}