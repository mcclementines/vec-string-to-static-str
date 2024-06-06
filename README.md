# vec-string-to-static-str 

`vec-string-to-static-str` is a Rust library providing utilities for converting vectors of `String`s into vectors of `&'static str`. 
This library includes both safe and unsafe methods for achieving this conversion.

## Features

- Safe conversion of `String` to `&'static str` with `Box::leak`
- Unsafe conversion of `String` to `&'static str` using `std::mem::transmute`

## Usage

Add `vec_string_to_static_str` to your `Cargo.toml`:

```toml
[dependencies]
vec-string-to-static-str = "1.0.0"
```
Make sure to add the `"unsafe"` feature flag to enable 
`unsafe_vec_string_to_static_str` if needed.

### Example

```rust
use vec_string_to_static_str::{vec_string_to_static_str, unsafe_vec_string_to_static_str};

fn main() {
    let strings = vec![String::from("hello"), String::from("world")];

    // Safe method
    let static_strs = vec_string_to_static_str(&strings);
    assert_eq!(static_strs, vec!["hello", "world"]);

    // Unsafe method
    let unsafe_static_strs = unsafe_vec_string_to_static_str(&strings);
    assert_eq!(unsafe_static_strs, vec!["hello", "world"]);
}
```

## Safety

- **Safe Method:** Uses `Box::leak` to convert `String` to `&'static str`, which leaks memory.
- **Unsafe Method:** Uses `std::mem::transmute` to extend the lifetime of string slices, which can cause undefined behavior if not used correctly.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
