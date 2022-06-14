use std::io;

use crate::account::{self, Account};

#[derive(Debug)]
pub struct Customer {
    pub cust_id: u32,
    pub name: String,
    pub accounts: Vec<Account>,
}
impl Customer {
    pub fn new(cust_id: u32, name: String) -> Customer {
        Customer {
            cust_id: cust_id,
            name: name,
            accounts: Vec::new(),
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
    //send money to another customer
}
