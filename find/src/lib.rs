use colored::*;
use std::fs;
use std::io;

#[derive(Debug)]
pub struct File {
    filename: String,
    filetype: String,
    sub_files: Vec<File>,
    content: String,
}

pub fn list_dirs(path: &str) -> io::Result<Vec<File>> {
    let mut vec_dirs: Vec<File> = vec![];
    for entry in fs::read_dir(path)? {
        let entry = entry.unwrap();
        let filename = entry.file_name().to_str().unwrap().to_string();
        let path = entry.path();
        if entry.file_type().unwrap().is_dir() {
            vec_dirs.push(File {
                filename,
                filetype: String::from("directory"),
                sub_files: list_dirs(path.to_str().unwrap()).unwrap(),
                content: String::new(),
            });
        } else {
            vec_dirs.push(File {
                filename: filename.clone(),
                filetype: String::from("file"),
                sub_files: vec![],
                content: fs::read_to_string(path.to_str().unwrap()).unwrap().to_string()
            });
        };
    }
    Ok(vec_dirs)
}

pub fn display(file: File, mut tab: u32) {
    let indent = (0..tab).map(|_| "   ").collect::<String>();
    if file.filetype == "directory" {
        println!("{indent}|--{}", file.filename.blue().bold());
        if file.sub_files.len() > 0 {
            tab += 1;
            for i in file.sub_files {
                display(i, tab);
            }
        }
    } else if file.filetype == "file" {
        println!("{indent}|--{}", file.filename.green().bold());
    }
}
