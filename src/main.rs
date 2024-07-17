pub mod fonts;
use std::io;
use std::io::Write;
use text_to_ascii_art::{align, to_art, Alignment};

fn main() {
    let font = "small";
    let alphabet1 = "abcdefghijklm".to_string();
    let alphabet2 = "nopqrstuvwzyx".to_string();
    let numbers = "0123456789".to_string();
    let symbols1 = "!\"#$%&'()*+,-./:".to_string();
    let symbols2 = ";<=>?@[\\]^_`{|}~".to_string();

    match to_art(alphabet1.to_uppercase(), font, 0, 0, 0) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }

    match to_art(alphabet2.to_uppercase(), font, 0, 0, 0) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }

    match to_art(alphabet1, font, 0, 0, 0) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }

    match to_art(alphabet2, font, 0, 0, 0) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }

    match to_art(numbers.to_uppercase(), font, 0, 0, 0) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }

    match to_art(symbols1.to_uppercase(), font, 0, 0, 0) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }

    match to_art(symbols2.to_uppercase(), font, 0, 0, 0) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }

    match to_art("Align Left".to_string(), font, 0, 0, 0) {
        Ok(string) => println!("{}", align(&string, Alignment::Left, 72)),
        Err(err) => println!("Error: {}", err),
    }

    match to_art("Align Center".to_string(), font, 0, 0, 0) {
        Ok(string) => println!("{}", align(&string, Alignment::Center, 72)),
        Err(err) => println!("Error: {}", err),
    }

    match to_art("Align Right".to_string(), font, 0, 0, 0) {
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

    match to_art(input, font, 0, 0, 0) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }
}
