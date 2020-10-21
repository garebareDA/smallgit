use smallgit::add;
use smallgit::common;
use smallgit::init;
use smallgit::commit;
use smallgit::tree;
use std::env;
use std::path::Path;

fn main() {
    return;
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        match init::init_create::create_init_file() {
            Ok(_) => println!("init complete"),
            Err(e) => eprintln!("{}", e),
        }
        return;
    }

    if args[1] == "add" {
        if !Path::new("./.smallgit").exists() {
            eprintln!("not found .smallgit run smallgit init");
            return;
        }

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
       let mut commit = commit::commit_file::CommitObject::new();
       let mut paths = common::serch_dir::SerchDir::new("./");
        paths.serch_dir().unwrap();
       match commit.commit_file(&paths.get_paths_dir()) {
           Ok(_) => {},
           Err(s) => {
               eprintln!("{}",s);
               return
           }
       }
    }
}
