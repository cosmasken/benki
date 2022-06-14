use crate::account::Account;
use crate::bank::Bank;
use crate::customer::Customer;
use std::collections::HashMap;
use std::io;
mod account;
mod bank;
mod customer;
mod utils;

fn main() {
    //show menu
    let accounts: Vec<Account> = Vec::new();
    let mut customers: Vec<Customer> = Vec::new();
    let bank = Bank::new("Bank of Rust".to_string());

    println!("Welcome to {}", bank.name);
    println!("Please select an option:");
    println!("1. Create new account");
    println!("2. Update account");
    println!("3. Delete account");
    println!("4. Deposit money");
    println!("5. Withdraw money");
    println!("6. Transfer money");
    println!("7. Print all Accounts & Customers");
    println!("8. Exit");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please type a number");
    match choice {
        1 => {
            println!("Please enter your name:");
            let mut name = String::new();
            io::stdin()
                .read_line(&mut name)
                .expect("Failed to read line");
            let name = name.trim();
            let mut customer = Customer::new(bank.customers.len() as u32 + 1, name.to_string());
            println!("Please enter your initial balance:");
            let mut balance = String::new();
            io::stdin()
                .read_line(&mut balance)
                .expect("Failed to read line");
            //add to customer_map
            let balance: i32 = balance.trim().parse().expect("Please type a number");
            let account = Account::new(utils::generate_account_number(), name.to_string(), balance);
            customer.add_account(account);
            customers.push(customer);
            println!("Account created successfully!");
            println!("{:?}", customers);
        }
        2 => {
            //update user name
            println!("Please enter your account number:");
            let mut account_number = String::new();
            io::stdin()
                .read_line(&mut account_number)
                .expect("Failed to read line");
            let account_number: u32 = account_number.trim().parse().expect("Please type a number");
            println!("Please enter your new name:");
            let mut name = String::new();
            io::stdin()
                .read_line(&mut name)
                .expect("Failed to read line");
            let name = name.trim();
            let mut customer = Customer::new(bank.customers.len() as u32 + 1, name.to_string());
            println!("Please enter your initial balance:");
            let mut balance = String::new();
            io::stdin()
                .read_line(&mut balance)
                .expect("Failed to read line");
            //add to customer_map
            let balance: i32 = balance.trim().parse().expect("Please type a number");
            let account = Account::new(utils::generate_account_number(), name.to_string(), balance);
            customer.add_account(account);
            customers.push(customer);
            //bank.customers.push(customer);
            //  customer_map = utils::save_customer_data(&bank.customers);
            println!("Account updated successfully!");
            println!("{:?}", customers);
        }
        3 => {
            //delete account from customer
            println!("Please enter your account number:");
            let mut account_number = String::new();
            io::stdin()
                .read_line(&mut account_number)
                .expect("Failed to read line");
            let account_number: u32 = account_number.trim().parse().expect("Please type a number");
            //print account number
            println!("Account number: {} deleted successfully!", account_number);
            //println!("Account {:?}deleted successfully!",customers);
        }
        4 => {
            //deposit money
            println!("Please enter your account number:");
            let mut account_number = String::new();
            io::stdin()
                .read_line(&mut account_number)
                .expect("Failed to read line");
            let account_number: u32 = account_number.trim().parse().expect("Please type a number");
       //enter amount to deposit
            println!("Please enter the amount to deposit:");
            let mut amount = String::new();
            io::stdin()
                .read_line(&mut amount)
                .expect("Failed to read line");
            let amount: i32 = amount.trim().parse().expect("Please type a number");
            //update account balance
            println!("Deposit of {} to account no {} successfull!",amount,account_number);
        }
        5 => {
            println!("Please enter your account number:");
            let mut account_number = String::new();
            io::stdin()
                .read_line(&mut account_number)
                .expect("Failed to read line");
            let account_number: i64 = account_number.trim().parse().expect("Please type a number");
            //enter amount to withdraw
            println!("Please Enter amount to withdraw");
            let mut amount = String::new();
            io::stdin()
                .read_line(&mut amount)
                .expect("Failed to read line");
            let amount: i32 = amount.trim().parse().expect("Please type a number");
            //update account balance
            println!("Withdraw of {} from account no {} successfull!",amount,account_number);

        }
        6 => {
            //transfer money
            println!("Please enter your account number:");
            let mut account_number = String::new();
            io::stdin()
                .read_line(&mut account_number)
                .expect("Failed to read line");
            let account_number: i64 = account_number.trim().parse().expect("Please type a number");
           //enter account to send to
            println!("Please enter the account number to send to:");
            let mut account_number_to = String::new();
            io::stdin()
                .read_line(&mut account_number_to)
                .expect("Failed to read line");
            let account_number_to: i64 = account_number_to.trim().parse().expect("Please type a number");
           
            //enter amount to transfer
            println!("Please Enter amount to transfer");
            let mut amount = String::new();
            io::stdin()
                .read_line(&mut amount)
                .expect("Failed to read line");
            let amount: i32 = amount.trim().parse().expect("Please type a number");

            //update account balance
            println!("Transfer of {} from account no {} to account no {} successfull!",amount,account_number,account_number_to);
        }
        7 => {
            //generate fake user data for testing
            let mut customer =
                Customer::new(bank.customers.len() as u32 + 1, "John Doe".to_string());
            let account = Account::new(
                utils::generate_account_number(),
                "John Doe".to_string(),
                100,
            );
            customer.add_account(account);
            customers.push(customer);
            let mut customer =
                Customer::new(bank.customers.len() as u32 + 1, "Jane Doe".to_string());
            let account = Account::new(
                utils::generate_account_number(),
                "Jane Doe".to_string(),
                200,
            );
            customer.add_account(account);
            customers.push(customer);
            let mut customer =
                Customer::new(bank.customers.len() as u32 + 1, "John Smith".to_string());
            let account = Account::new(
                utils::generate_account_number(),
                "John Smith".to_string(),
                300,
            );
            customer.add_account(account);
            customers.push(customer);
            //print all users
            println!("All Customers:");
            //print customers
            println!("{:?}", customers);
            println!("All Accounts:");
            //print accounts
            println!("{:?}", accounts);
        }
        8 => {
            //confirm if user wants to exit
            println!("Are you sure you want to exit? (y/n)");
            let mut answer = String::new();
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line");
            let answer: String = answer.trim().to_string();
            if answer == "y" {
                println!("Thank you for using {}", bank.name);
                std::process::exit(0);
            }else {
                
            }
        }

        _ => {
            println!("Invalid choice");
        }
    }
}
