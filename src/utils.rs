use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::File;
use std::ffi::OsStr;

pub fn hello() {
    println!("Hello World!");
}

pub fn open_file<P: AsRef<Path>>(path: P) -> File {
    let path = path.as_ref();

    let file = File::open(path).unwrap();

    file
}

pub struct Content {
    pub text: String,
    pub attributes: Option<String>,
}

// TODO: remove this and use Path::relative_from when stable
fn normalize_path_str(s: String) -> String {
    let s = s.to_owned();

    if s.ends_with("/") {
        s
    } else {
        s + "/"
    }
}

pub fn new_path<P: AsRef<Path>>(path: &P, prefix: Option<String>) -> PathBuf {
    let path = path.as_ref();
    let file_name = path.file_name().unwrap_or(OsStr::new(""));
    let normalized_source = normalize_path_str("blah".to_string());
    let rp = path.to_str().unwrap().replace(&normalized_source, "");
    let base = PathBuf::from("destination");

    let relativized_path = if path.starts_with(normalized_source) {
        Path::new(&rp)
    } else {
        path
    };

    let new_path = match prefix {
        Some(p) => base.join(p).join(file_name),
        None => base.join(relativized_path),
    };

    new_path
}
