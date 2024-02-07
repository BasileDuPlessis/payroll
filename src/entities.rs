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
}
