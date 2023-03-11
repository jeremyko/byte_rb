# byte_rb

byte ring buffer for rust.


https://crates.io/crates/byte_rb

[Documentation](https://docs.rs/byte_rb)

## Installation

To use `byte_rb`, first add this to your `Cargo.toml`:

```toml
[dependencies]
byte_rb = "1"
```
Or Run the following Cargo command in your project directory:

```
cargo add byte_rb
```

## Usage

```rust
use byte_rb::BrBuffer;

let mut cbuf = BrBuffer::new(6);
println!("{:?}", cbuf);

assert!(cbuf.append(6, b"123456").unwrap());
assert_eq!(cbuf.rpos(), 0);
assert_eq!(cbuf.wpos(), 6);
// "123456"
let result = cbuf.get(3).unwrap();
assert_eq!(result, b"123");
assert_eq!(cbuf.cumulated_len(), 3);
assert_eq!(cbuf.rpos(), 3);
assert_eq!(cbuf.wpos(), 6);
// "  456"

assert!(cbuf.append(3, b"789").unwrap());
assert_eq!(cbuf.cumulated_len(), 6);
assert_eq!(cbuf.rpos(), 3);
assert_eq!(cbuf.wpos(), 3);
// "789456"

let result = cbuf.get(1).unwrap();
assert_eq!(result, b"4");
assert_eq!(cbuf.rpos(), 4);
assert_eq!(cbuf.wpos(), 3);
assert_eq!(cbuf.cumulated_len(), 5);
// "789 56"

let result = cbuf.get(5).unwrap();
assert_eq!(result, b"56789");
assert_eq!(cbuf.rpos(), 3);
assert_eq!(cbuf.wpos(), 3);
assert_eq!(cbuf.cumulated_len(), 0);

assert_eq!(cbuf.capacity(), 6);
```

## License

This project is licensed under the [MIT license](LICENSE).