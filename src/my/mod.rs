/*******************************************************************************
Title           : mod.rs
Author          : Keisuke Suzuki
Created on      : 9/25/19
Modification    :
*******************************************************************************/
extern crate mysql;
pub mod config;

use mysql::{Opts,OptsBuilder,Pool};
use config::Env;


pub struct Mysql{
    pub(super) env: Env,
    pool: Pool
}

fn get_opts(env:&Env)->Opts{
    let mut builder = OptsBuilder::new();
    builder.ip_or_hostname(Some(env.host())).user(Some(env.user()))
           .pass(Some(env.password())).db_name(Some(env.database()));
    Opts::from(builder)
}

impl Mysql {
    pub fn new() -> Result<Mysql, mysql::Error>{
        let env=Env::get_env();
        let pool=Pool::new(get_opts(&env))?;
        Ok( Mysql{ env, pool } )
    }
    
    pub fn prep_exec(&self, query: String) -> Result<(), mysql::Error> {
        self.pool.prep_exec(query, ())?;
        Ok(())
    }
}