use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug)]
pub enum PayType {
    Salaried,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Employee {
    pub name: String
}

impl Employee {
    pub fn new(name: &str) -> Employee {
        Employee {
            name: name.to_owned(),
        }
    }
}
