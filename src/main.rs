use std::{
     process,
};

fn main() {
    if let Err(e) = typester::run() {
        println!("Application error: {e}");
        process::exit(1);
    }
}
