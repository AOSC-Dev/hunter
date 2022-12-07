use file_handler::hunter;

use crate::package_info::to_tabled;

mod file_handler;
mod package_info;
mod parser;

fn main() {
    let list = hunter().unwrap();
    if !list.is_empty() {
        let table = to_tabled(&list).unwrap();
    
        println!("{}", table);
    }
}
