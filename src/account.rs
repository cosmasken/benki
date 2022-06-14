#[derive(Debug)]

pub struct Account {
    pub number: u32,
    pub name: String,
    pub balance: i32,
}

impl Account {
    pub fn copy(&self) -> Account {
        Account {
            number: self.number,
            name: self.name.clone(),
            balance: self.balance,
        }
    }
    pub fn new(number: u32, name: String, balance: i32) -> Account {
        Account {
            number: number,
            name: name,
            balance: balance,
        }
    }
}
