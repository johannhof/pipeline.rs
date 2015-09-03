# pipeline.rs

Pipeline is a macro to pipe your functions calls, like in F# or Elixir.

```
let body = pipe!("http://rust-lang.org" => download => parse => [unwrap])
```

```
    fn times(a: u32, b: u32) -> u32{
        return a * b * c;
    }

    let num = pipe!(
      4
      => (times(10))
      => (times(4))
    )
```

