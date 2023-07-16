## Usage

Provides 2 functions for working with and creating ascii art from simple text.

---

Convert text and selected symbols into ascii art

```rust
fn convert (input: String) -> Result<String, String>
```

---

Join 2 ascii art images together (they both need to be rectangle for it to work, add spaces to the end so it's rectangle)

```rust
fn join_art (s1: &str, s2: &str) -> String
```

---

If you cargo run this crate you can enter input text and it will just print and than exit.

<video width="640" height="480" controls>
  <source src="./asset/movie.mp4" type="video/mp4">
</video>

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
text-to-ascii-art="0.1.5"
```

## Road map

- [x] kindof monospace
- [x] full monospace
- [ ] add some more special characters
- [ ] auto spacing for join art
- [ ] better documentation
