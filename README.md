# pipeline.rs [![](https://travis-ci.org/johannhof/pipeline.rs.svg)](https://travis-ci.org/johannhof/pipeline.rs) [![](https://img.shields.io/crates/v/pipeline.svg)](https://crates.io/crates/pipeline)

Pipeline is a macro collection to pipe your functions calls, like in F# or Elixir. Instead of the nice `|>` operator it uses `=>` as a pipe character, due to limitations in the Rust macro system. 

## Usage

Put this in your Cargo.toml
```toml
[dependencies]

pipeline = "0.6.0"
```

Then you can import the macros with extern crate and macro_use
```rust
#[macro_use]
extern crate pipeline;
```

## Examples

```rust
let result = pipe!("http://rust-lang.org" => download => parse => get_links)
```

```rust
fn times(a: u32, b: u32) -> u32{
    return a * b;
}

let num = pipe!(
  4
  => _.times(_, 10)
  => |i: u32| i * 2
  => _ * 4
);

// takes a string length, doubles it and converts it back into a string
let length = pipe!(
    "abcd"
    => _.len
    => _ as u32
    => times(_, 2)
    => ToString::to_string
);
```


## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
