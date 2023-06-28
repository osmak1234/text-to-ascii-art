pub mod ascii;
use std::io;
use text_to_ascii_art::convert;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    for i in 0..100 {
        input = i.to_string();
        match convert(input) {
            Ok(string) => println!("{}", string),
            Err(err) => println!("Error: {}", err),
        }
    }
}
