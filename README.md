# pipeline.rs

Pipeline is a macro collection to pipe your functions calls, like in F# or Elixir.

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
  => (times(4))
)
```

