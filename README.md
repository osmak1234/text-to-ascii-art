## Usage

I made this crate for my project, if you need larger text in terminal, you need to use
ascii art characters. This simple crate provides function to make a ascii art string from text and to join 2 ascii art strings (they both need to be rectangle for it to work).

fn convert (input: String) -> Result<String, String>

If you cargo run this crate you can enter input text and it will just print and than exit.

## <a href="#-usage-in-rust"><img src="https://rustacean.net/assets/rustacean-flat-noshadow.svg" width="16" height="16"></a> Usage in Rust

```rust
use text_to_ascii_art::convert;

fn main () {
  match convert("Hello in ascii".to_string()) {
      Ok(string) => println!("{}", string),
      Err(err) => println!("Error: {}", err),
  }
}
```

```toml
[dependencies]
text-to-ascii-art="0.1"
```

## TODO

- [x] kindof monospace
- [ ] full monospace
- [ ] add some more characters
- [ ] eliminate the need for spacing
