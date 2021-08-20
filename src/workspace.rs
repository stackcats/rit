use std::fs;
use std::path::PathBuf;

static IGNORE: [&str; 3] = [".", "..", ".git"];

pub struct Workspace {
    path: PathBuf,
}

impl Workspace {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn list_files(&self) {
        let paths = fs::read_dir(self.path.clone()).unwrap();

        for path in paths {
            let p = path.unwrap().path();
            let s = p.file_name().unwrap();
            if !IGNORE.contains(&s.to_str().unwrap()) {
                println!("{}", p.display());
            }
        }
    }
}
