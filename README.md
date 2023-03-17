# isnt-even-nor-odd

Returns true if the given number isn't even nor odd.

## Install

Specify the dependency in Cargo.toml:

```yaml
[dependencies]
isnt-even-nor-odd = "1"
```

Fetch it with cargo:

```bash
cargo build
```

## Usage

```rust
use isnt_even_nor_odd::IsntEvenNorOdd;

let _i : i32 = 1;
println!("{}", _i.isnt_even_nor_odd()); // prints false
```

## About

### License

Credit to [purewhite](https://github.com/PureWhiteWu) because i just copy pasted all of this and reversed his dependency. the logic was too hard to understand for me so thakns for the boilerplate

Released under the [MIT License](LICENSE).
