use colored::*;
use find::{display, list_dirs};

fn main() {
    let path = "/home";
    let list = list_dirs(path).unwrap();
    println!(
        "{} files found in {}\n",
        list.len().to_string().blue(),
        path.red()
    );
    for i in list {
        display(i, 0);
    }
}
