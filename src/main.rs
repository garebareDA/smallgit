use smallgit::add;
use smallgit::cat_file;
use smallgit::commit;
use smallgit::common;
use smallgit::init;
use smallgit::tree;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("args error");
        return;
    }

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

        return;
    }

    if args[1] == "commit" {
        let mut commit = commit::commit_file::CommitObject::new();
        match commit.commit_file() {
            Ok(()) => {
                println!("commit! {}", commit.get_hash());
            }
            Err(s) => {
                eprintln!("{}", s);
                return;
            }
        }
        return;
    }

    if args[1] == "tree" {
        if args.len() != 3 {
            println!("smallgit tree [hash]");
            return;
        }
        let mut tree = tree::tree_git_object::CommitGet::new();
        tree.set_hash(&args[2]);
        match tree.tree_main() {
            Ok(_) => {}
            Err(s) => {
                eprintln!("{}", s);
                return;
            }
        }
        tree.object_display();
        return;
    }

    if args[1] == "clone" {
        if args.len() != 3 {
            println!("smallgit clone [hash]");
            return;
        }
        let mut tree = tree::tree_git_object::CommitGet::new();
        tree.set_hash(&args[2]);
        match tree.tree_main() {
            Ok(_) => {}
            Err(s) => {
                eprintln!("{}", s);
                return;
            }
        }
        match tree.object_clone() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e.to_string());
            }
        }
        return;
    }

    if args[1] == "cat-file" {
        if args.len() != 3 {
            println!("smallgit cat-file [hash]");
            return;
        }
        match cat_file::display_file::display(&args[2]) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        }
        return;
    }

    if args[1] == "status" {
        match common::index_readed::read_index() {
            Ok(indexs) => {
                for index in indexs {
                    println!("{} {}", index.status, index.path);
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        }
        return;
    }

    println!("command not found");
}
