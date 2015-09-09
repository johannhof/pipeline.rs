# pipeline.rs [![](https://travis-ci.org/johannhof/pipeline.rs.svg)](https://travis-ci.org/johannhof/pipeline.rs) [![](https://img.shields.io/crates/v/pipeline.svg)](https://crates.io/crates/pipeline)

Pipeline is a macro collection to pipe your functions calls, like in F# or Elixir. Instead of the nice `|>` operator it uses `=>` as a pipe character, due to limitations in the Rust macro system. 

## Usage

Put this in your Cargo.toml
```toml
[dependencies]

pipeline = "0.4.1"
```

Then you can import the macros with extern crate and macro_use
```rust
#[macro_use]
extern crate pipeline;
```

## Examples

```rust
// pipe_res exits the pipeline early if a function returns an Err()
let result = pipe_res!("http://rust-lang.org" => download => parse => get_links)
```

```rust
fn times(a: u32, b: u32) -> u32{
    return a * b;
}

let num = pipe!(
  4
  => (times(10))
  => {|i: u32| i * 2}
  => (times(4))
);

// takes a string length, doubles it and converts it back into a string
let length = pipe!(
    "abcd"
    => [len]
    => (as u32)
    => times(2)
    => [to_string]
);
```

## Syntax Features

Any `pipe` starts with an expression as initial value and requires you
to specify a function to transform that initial value.
```rust
let result = pipe!(2 => times2);
```

You can get more fancy with functions, too, if you add parentheses like
in a normal function call, the passed parameters will be applied to that
function after the transformed value.

> You have to put it in parentheses
because the Rust macro system can be very restrictive.
If you figure out a way to do it without please make a PR.

```rust
let result = pipe!(2 => (times(2)));
```

You can pass closures \o/! A closure must be wrapped in curly brackets (`{}`)
```rust
let result = pipe!(
  2
  => (times(2))
  => {|i: u32| i * 2}
);
```

If you want a function to be called as a method on the transform value,
put it in square brackets (`[]`).
```rust
let result = pipe!(
    "abcd"
    => [len]
);
```
