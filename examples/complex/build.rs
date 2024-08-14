use std::{env, fs, path::PathBuf};

const COPY_DIR: &'static str = "data";

fn copy_dir(from: PathBuf, to: PathBuf) {
    if to.exists() {
        fs::remove_dir_all(&to).unwrap();
    }

    fs::create_dir(&to).unwrap();

    for path in fs::read_dir(from).unwrap() {
        let path = path.unwrap().path();
        let to = to.clone().join(path.file_name().unwrap());

        if path.is_file() {
            fs::copy(&path, to).unwrap();
        } else if path.is_dir() {
            if !to.exists() {
                fs::create_dir(&to).unwrap();
            }

            copy_dir(path, to);
        }
    }
}

fn main() {
    let out = PathBuf::from(format!(
        "target/{}/{}",
        env::var("PROFILE").unwrap(),
        COPY_DIR
    ));

    copy_dir(COPY_DIR.into(), out);

    println!("cargo:rerun-if-changed=.");
}
