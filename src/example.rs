/*******************************************************************************
Title           : example.rs
Author          : Keisuke Suzuki
Created on      : 9/25/19

*******************************************************************************/
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::Write;


#[derive(Debug, PartialEq, Eq)]
pub struct Example {
    id: i32,
    speed: i32,
    dist: i32,
}

pub fn get_examples()->Vec<Example> {
    vec![
        Example { id: 1, speed: 2, dist: 93 },
        Example { id: 3, speed: 4, dist: 354 },
        Example { id: 5, speed: 6, dist: 43 },
        Example { id: 7, speed: 8, dist: 26 },
        Example { id: 9, speed: 10, dist: 78 },
    ]
}

pub fn insert_examples() -> Result<(), Box<dyn Error>> {
 
    println!("running with a sample file");
    let example = get_examples();
    let mut write_str = Vec::new();
    let example_type = "int,int,int";
    let data_file_path = Path::new("example.csv");
    let type_file_path = Path::new("type.csv");
    
    //write two files
    let mut outfile = File::create(&data_file_path)?;
    writeln!(&mut write_str, "{}", "id,speed,dist")?;
    for it in example.iter() {
        writeln!(&mut write_str, "{},{},{},", it.id, it.speed, it.dist)?;
    }
    outfile.write_all(&write_str)?;
    outfile = File::create(&type_file_path)?;
    write_str.clear();
    writeln!(&mut write_str, "{}", example_type)?;
    outfile.write_all(&write_str)?;
    Ok(())
}