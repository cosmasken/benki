use std::fs::File;

use rand::prelude::*;

//generate u32 account number
pub fn generate_account_number() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(100000000, 999999999)
}
