use smallgit::add;
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
        let mut add_file = add::add_files::AddFile::new(path);
        match add_file.add_file() {
            Ok(_) => {}
            Err(e) => eprintln!("{}", e),
        }
    }
}
