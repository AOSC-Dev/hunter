use file_handler::hunter;

mod file_handler;

fn main() {
    let list = hunter();

    if let Ok(list) = list {
        for i in list {
            println!("{}", i);
        }
    }
}
