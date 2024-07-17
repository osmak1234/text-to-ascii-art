pub mod fonts;
use std::io;
use std::io::Write;
use text_to_ascii_art::{align, convert, Alignment};

fn main() {
    let font = "small";
    let alphabet1 = "abcdefghijklm".to_string();
    let alphabet2 = "nopqrstuvwzyx".to_string();
    let numbers = "0123456789".to_string();
    let symbols1 = "!\"#$%&'()*+,-./:".to_string();
    let symbols2 = ";<=>?@[\\]^_`{|}~".to_string();

    match convert(alphabet1.to_uppercase(), font, 0, 0, 0) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }

    match convert(alphabet2.to_uppercase(), font, 0, 0, 0) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }

    match convert(alphabet1, font, 0, 0, 0) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }

    match convert(alphabet2, font, 0, 0, 0) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }

    match convert(numbers.to_uppercase(), font, 0, 0, 0) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }

    match convert(symbols1.to_uppercase(), font, 0, 0, 0) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }

    match convert(symbols2.to_uppercase(), font, 0, 0, 0) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }

    match convert("Align Left".to_string(), font, 0, 0, 0) {
        Ok(string) => println!("{}", align(&string, Alignment::Left, 72)),
        Err(err) => println!("Error: {}", err),
    }

    match convert("Align Center".to_string(), font, 0, 0, 0) {
        Ok(string) => println!("{}", align(&string, Alignment::Center, 72)),
        Err(err) => println!("Error: {}", err),
    }

    match convert("Align Right".to_string(), font, 0, 0, 0) {
        Ok(string) => println!("{}", align(&string, Alignment::Right, 72)),
        Err(err) => println!("Error: {}", err),
    }

    print!("\n\nType your own string: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input = input.trim().to_string();

    match convert(input, font, 0, 0, 0) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }
}
