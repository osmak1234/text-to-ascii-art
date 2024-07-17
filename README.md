## Usage

Provides some functions for working with and creating ascii art from simple text.

---

Convert text and symbols into ascii art

```rust
fn to_art (input: String, leading: usize, gap: usize, trailing: usize) -> Result<String, String>
```

---

To align the art within some space, please use `align` function with `Alignment` enum
`width` parameter represents space for your art that will be aligned inside it

```rust
fn align(art: &str, alignment: Alignment, width: usize) -> String
```

---

Join 2 ascii art images together (they both need to be same width and height rectangle to work, add spaces to the end so it's rectangle)

`gap` parameter sets number of spaces between arts to be joined

```rust
fn join_art (s1: &str, s2: &str, gap: usize) -> String
```

---

If you run this crate, it will print ascii symbols with 'small' named font and waits you enter your own string
for previewing

https://github.com/osmak1234/text-to-ascii-art/assets/91377215/ea937074-fdc6-4c67-839e-15d6854f0bee

## <a href="#-usage-in-rust"><img src="https://rustacean.net/assets/rustacean-flat-noshadow.svg" width="16" height="16"></a> Usage in Rust

main.rs

```rust
use textart::to_art;

fn main () {
  match to_art("Hello in ascii".to_string()) {
      Ok(string) => println!("{}", string),
      Err(err) => println!("Error: {}", err),
  }
}
```

Cargo.toml

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
