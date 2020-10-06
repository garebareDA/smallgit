use std::env;
use std::fs;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        match create_init_file() {
            Err(e) => eprintln!("{}", e),
            Ok(_) => println!("init complete"),
        }
        return;
    }

    if args[2] == "add" {
        
    }
}

fn create_init_file() -> Result<(), String> {
    match fs::create_dir("./.smallgit") {
        Ok(_) => {}
        Err(_) => {
            return Err("failed to create dir".to_string());
        }
    }

    match File::create("./.smallgit/index") {
        Ok(_) => {}
        Err(_) => {
            return Err("failed to create  file".to_string());
        }
    }

    match fs::create_dir("./.smallgit/refs") {
        Ok(_) => {}
        Err(_) => {
            return Err("failed to create dir".to_string());
        }
    }

    match fs::create_dir("./.smallgit/objects") {
        Ok(_) => {}
        Err(_) => {
            return Err("failed to create dir".to_string());
        }
    }

    return Ok(());
}
