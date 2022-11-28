use file_handler::hunter;

use crate::package_info::dpkg_info;

mod file_handler;
mod package_info;

fn main() {
    let list = hunter();

    if let Ok(list) = list {
        for i in list {
            println!("{:?}", dpkg_info(&i));
        }
    }
}
