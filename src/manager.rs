//create account manager struct
#[derive(Debug)]
pub struct Manager {
    pub employee_id: u32,
    pub name: String,
}
//implementation
pub impl Manager {
    pub fn new(employee_id: u32, name: String) -> Manager {
        Manager {
            employee_id: employee_id,
            name: name,
        }
    }
}
