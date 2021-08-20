use std::collections::VecDeque;
use std::env;
use std::fs;

mod workspace;
use crate::workspace::*;

fn main() {
    let mut args: VecDeque<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        println!("no command");
        return;
    }
    let command = args.pop_front().unwrap();
    match command.as_str() {
        "init" => {
            let mut dir_path = env::current_dir().unwrap();
            if let Some(path) = args.get(0) {
                dir_path.push(path);
            }
            let mut git_path = dir_path.clone();
            git_path.push(".git");
            for dir in ["objects", "refs"].iter() {
                let mut path = git_path.clone();
                path.push(dir);
                if let Err(e) = fs::create_dir_all(path) {
                    eprintln!("fatal: {}", e.to_string());
                    std::process::exit(1);
                }
            }
            println!("Initialized empty rit repository in {}", dir_path.display());
            std::process::exit(0);
        }
        "commit" => {
            let root_path = env::current_dir().unwrap();
            let mut git_path = root_path.clone();
            git_path.push(".git");
            let mut db_path = git_path.clone();
            db_path.push("objects");
            let ws = Workspace::new(root_path);
            ws.list_files();
            std::process::exit(0);
        }
        _ => {
            println!("rit: '#{{ {} }}' is not a rit command.", command);
            std::process::exit(1);
        }
    }
}
