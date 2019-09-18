/*******************************************************************************
Title           : main.rs
Author          : Keisuke Suzuki
Created on      : Sep 17, 2019
Description     : rust practice which inserts a csv.file to mysql server on
localhost
Usage           : cargo run src/main.rs <path to the csv file> <path to the
                    type specifier>
                  on project root for now
*******************************************************************************/
extern crate mysql;
use mysql as my;

use dotenv;
use std::env;
use my::Opts;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use std::process::Command;
use std::string::String;

#[derive(Clone,Debug)]
struct MyValue{
    host: String,
    user: String,
    password: String,
    database: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Example{
    id: i32,
    speed: i32,
    dist: i32
}

fn main() {
    
    dotenv::dotenv().ok();
    let env=MyValue{
        host: dotenv::var("HOST").unwrap(),
        user: dotenv::var("USER").unwrap(),
        password:dotenv::var("PASSWORD").unwrap(),
        database:dotenv::var("DATABASE").unwrap()
    };
    
    let mut builder=my::OptsBuilder::new();
    builder.ip_or_hostname(Some(env.host.clone())).user(Some(env.user.clone()))
        .pass(Some(env.password.clone())).db_name(Some(env.database.clone()));
    
    
    let mut valid_args=true;
    let opts=Opts::from(builder);
    let pool=my::Pool::new(opts).unwrap();
    
    let mut write_str=Vec::new();
    let args: Vec<String> = env::args().collect();
    let mut arg_one = String::new();
    let mut arg_two = String::new();
    if args.len()==3 {
        valid_args=false;
        println! ("\tthis command requires either non or two argument(s).\n\
                   \t\tusage: mysqlinsertcsv <path to the csv file> <path to \
                              the type specifier>");
    }
    else if args.len()!=2 {
        println!("{:?}", args);
        arg_one = args[2].clone();
        arg_two = args[3].clone();
    }
    //noargs passed, create sample file
    else {
        println!("running with a sample file");
        let example = vec![
            Example { id: 1, speed: 2, dist: 93 },
            Example { id: 3, speed: 4, dist: 354 },
            Example { id: 5, speed: 6, dist: 43 },
            Example { id: 7, speed: 8, dist: 26 },
            Example { id: 9, speed: 10, dist: 78 },
        ];
        let example_type="int,int,int";
        let data_file_path = Path::new("example.csv");
        let type_file_path= Path::new("type.csv");
        {   //write two files
            let mut outfile = File::create(&data_file_path).expect("unable to write");
            writeln!(&mut write_str, "{}", "id,speed,dist").unwrap();
            for it in example.iter() {
                writeln!(&mut write_str, "{},{},{},", it.id, it.speed, it.dist)
                    .unwrap();
            }
            outfile.write_all(&write_str).expect("unable to write");
            outfile=File::create(&type_file_path).expect("unable to write");
            write_str.clear();
            writeln!(&mut write_str,"{}",example_type).unwrap();
            outfile.write_all(&write_str).expect("unable to write");
        }
        arg_one = "example.csv".to_string();
        arg_two = "type.csv".to_string();
    }
    if valid_args {
        //read first line of datafile and typefile
        let data_file = format!("{}", arg_one);
        let type_file = format!("{}", arg_two);
        let data_path = Path::new(&data_file);
        let type_path = Path::new(&type_file);
        let data_file = File::open(data_path).expect("unable to open");
        let type_file = File::open(type_path).expect("unable to open");
        let data_line = BufReader::new(data_file).lines().nth(0).unwrap().unwrap();
        let fields: Vec<&str> = data_line.split(',').collect();
        let type_line = BufReader::new(type_file).lines().nth(0).unwrap()
            .unwrap();
        let types: Vec<&str> = type_line.split(',').collect();
    
        if fields.len() != types.len() {
            println!("invalid field or type information");
            valid_args = false;
        }
        if valid_args {
            //get table name from the csv file to create the table
            let temp: Vec<&str> = arg_one.rsplit(|x| x == '/' || x == '.')
                .collect();
            let table_name = temp[1];
            let table_query = format!("{}", format!("create table {}(", table_name));
            let mut field_query = String::new();
            for i in 0..fields.len() {
                let mut temp = format!("{} {} not null,", fields[i], types[i]);
                if i == fields.len() - 1 { temp.pop(); }
                field_query.push_str(temp.as_str());
            }
            let query = format!("{}{})", table_query, field_query);
            println!("{}", query);
            pool.prep_exec(query, ()).unwrap();
        
        
            //insert table on mysql server
            let user_command = format!("--user={}", env.user);
            let pass_command = format!("--password={}", env.password);
            Command::new("mysqlimport")
                .arg("--ignore-lines=1")
                .arg("--fields-terminated-by=,")
                .arg("--local")
                .arg(user_command)
                .arg(pass_command)
                .arg(env.database)
                .arg(arg_one)
                .status()
                .expect("failed to execute process");
        }
    }
}
