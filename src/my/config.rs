/*******************************************************************************
Title           : config.rs
Author          : Keisuke Suzuki
Created on      : 9/25/19
Modification    :
*******************************************************************************/
extern crate dotenv;
extern crate rpassword;
use::rpassword::*;


#[derive(Clone, Debug)]
pub struct Env {
    host: String,
    user: String,
    password: String,
    database: String,
}

impl Env {
    fn get_password() -> String {
        prompt_password_stdout("Enter your user password: ").unwrap()
    }
    pub fn get_env() -> Env {
        dotenv::dotenv().ok();
        let pass = Env::get_password();
        Env {
            host: dotenv::var("HOST").unwrap(),
            user: dotenv::var("USERNAME").unwrap(),
            password: pass,
            database: dotenv::var("DATABASE").unwrap(),
        }
    }
    pub fn host(&self) -> &str { &self.host }
    pub fn user(&self) -> &str { &self.user }
    pub fn password(&self) -> &str { &self.password }
    pub fn database(&self) -> &str { &self.database }
}