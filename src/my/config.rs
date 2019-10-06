/*******************************************************************************
Title           : config.rs
Author          : Keisuke Suzuki
Created on      : 9/25/19
Modification    :
*******************************************************************************/
extern crate dotenv;
extern crate rpassword;
use::dotenv::*;
use::rpassword::*;

#[derive(Clone, Debug)]
pub struct Env {
    host: String,
    user: String,
    password: String,
    database: String,
}

// ask user for password on stdout invisibly, and returns the password entered
fn get_password() -> String {
    prompt_password_stdout("Enter your MySQL password: ").unwrap()
}

impl Env {
    // reads env from .env return itself
    pub fn get_env() -> Env {
        dotenv::dotenv().ok();
        Env {
            host: var("HOST").unwrap(),
            user: var("USERNAME").unwrap(),
            password: get_password(),
            database: var("DATABASE").unwrap(),
        }
    }
    // getters
    pub fn host(&self) -> &str { &self.host }
    pub fn user(&self) -> &str { &self.user }
    pub fn password(&self) -> &str { &self.password }
    pub fn database(&self) -> &str { &self.database }
}
