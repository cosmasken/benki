use crate::{ customer::Customer};
use crate::{account::Account};


#[derive(Debug)]
pub struct Bank {
    pub name: String,
    pub customers: Vec<Customer>,
}

impl Bank {
    pub fn new(name: String) -> Bank {
        Bank {
            name: name,
            customers: Vec::new()
        }
    }
}
