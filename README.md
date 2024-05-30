# lazier\_static

## Description

A thin layer around `std::sync::OnceLock` providing cached, lazy, static initialization.

## Examples

```rust
use lazier_static::lazier_static;

lazier_static! {
    fn hello_world() -> &str {
        "Hello, World!"
    }

    fn number() -> i32 {
        10 * 32
    }
}

fn main() {
    println!("{}", number());
}
```

## Usage

### `cargo` command

`cargo add lazier_static`

### Cargo.toml

`lazier_static = "0.1.0"`

## License

MIT OR Apache-2.0
