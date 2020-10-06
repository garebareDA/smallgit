use std::env;
use smallgit::init;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        match init::init_create::create_init_file() {
            Err(e) => eprintln!("{}", e),
            Ok(_) => println!("init complete"),
        }
        return;
    }

    if args[1] == "add" {
        if args.len() != 3 {
            return;
        }
        let path = &args[3];
    }
}