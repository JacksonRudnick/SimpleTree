use std::{fs, env, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = Path::new(&args[1]);

    iter_dirs(&path);
}

fn iter_dirs(dir: &Path) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                iter_dirs(&path);
            } else {
                println!("{:?} {:?}", path, entry);
            }
        }
    }
}