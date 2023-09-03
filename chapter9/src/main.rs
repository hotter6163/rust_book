mod read_username_from_file;

use read_username_from_file::read_username_from_file;
use std::{fs::File, io::ErrorKind};

fn main() {
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    let s = read_username_from_file().unwrap();
    println!("{}", s)

    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
    //         Ok(fc) => fc,
    //         Err(e) => {
    //             panic!("Tried to create file but there was a problem: {:?}", e)
    //         }
    //     },
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error)
    //     }
    // };
}
