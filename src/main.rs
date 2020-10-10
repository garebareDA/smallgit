use smallgit::add;
use smallgit::common;
use smallgit::init;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        match init::init_create::create_init_file() {
            Ok(_) => println!("init complete"),
            Err(e) => eprintln!("{}", e),
        }
        return;
    }

    if args[1] == "add" {
        if args.len() != 3 {
            return;
        }
        let path = &args[2];
        let mut paths = common::serch_dir::SerchDir::new(path);
        paths.serch_dir().unwrap();
        match add::add_files::write_index(paths) {
            Ok(()) => {}
            Err(s) => {
                eprintln!("{}", s);
                return;
            }
        }

        match add::add_files::create_objects() {
            Ok(()) => {}
            Err(s) => {
                eprintln!("{}", s);
                return;
            }
        }
    }

    if args[1] == "commit" {
        
    }
}
