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

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct Commands {
    user: String,
    pass: String,
    data: String,
    sep : String,
    ter : String,
    opt : String,
}

pub struct MysqlInsert{
    env: Env,
    pool: Pool
}

impl MysqlInsert {
    // constructor which returns itself
    pub fn new() -> BoxResult<MysqlInsert> {
        let env = Env::get_env();
        let pool = Pool::new(Self::get_opts(&env))?;
        Ok(MysqlInsert { env, pool })
    }
    // create table with passed query
    pub fn prep_exec(&self, query: String) -> BoxResult<()> {
        self.pool.prep_exec(query, ())?;
        Ok(())
    }
    // Insert data using mysql command 'mysqlimport'
    pub fn insert(&self, file_name: &str, sep: &str, ter: &str)
            -> BoxResult<()>{
        let commands = self.get_commands(sep, ter);

        cmd!(mysqlimport ("--local") ("--ignore-lines=1")
                (commands.sep)
                (commands.ter)
                (commands.opt)
                (commands.user)
                (commands.pass)
                (commands.data)
                (file_name) )
            .status()
            .expect("failed to execute process");
        Ok(())
    }
    //get command args for 'mysqlimport'
    fn get_commands(&self, sep: &str, ter: &str) -> Commands {
        Commands {
            user: format!("--user={}", self.env.user()),
            pass: format!("--password={}", self.env.password()),
            data: self.env.database().to_string(),
            sep : format!("--fields-terminated-by={}", sep),
            ter : format!("--lines-terminated-by={}", ter),
            opt : "--fields-optionally-enclosed-by=\"".to_string()
        }
    }
    //builds opts from builder
    fn get_opts(env:&Env)->Opts {
        let mut builder = OptsBuilder::new();
        builder.ip_or_hostname(Some(env.host()))
               .user(Some(env.user()))
               .pass(Some(env.password()))
               .db_name(Some(env.database()));
        Opts::from(builder)
    }
}

