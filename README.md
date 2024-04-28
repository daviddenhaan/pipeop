# The pipe operator in Rust

This crate is exactly what you expect it to be, the pipe operator in Rust.

## Usage

You can construct a "pipeline" by passing any expression and at least a
single pipe into the `::pipeop::pipe!` macro. There are some special things
you can do but, in its most basic form the macro tries to literally call
your pipe with the item in the pipeline.

```rust
const fn add_one(to: i32) -> i32 {
    to + 1
}

let result = pipe!(1 |> add_one |> add_one);
assert!(result, 3);
```

### Invoking methods on the item in the pipeline

You can invoke methods on the item in the pipeline at any time by 
prefixing the pipe with a `.`.

This example calls the `add` method on the item in the pipelines
with `1` as the single argument.

```rust
use std::ops::Add;
pipe!(1 |> .add(1));
```

### Closure based pipes

You can also use closures as pipes, so you don't have to define a
whole new function for every simple operation. Both types of closures are valid, you can have a closure that just
evaluates an expression, or you can have a whole block of code.

```rust
pipe!("Hello!"
    |> .to_uppercase()
    |> |item| println!("{}", item)
);
```

You can also make closure based pipes look a little nicer
by using this syntax instead.

```rust
pipe!("Hello!"
    |> .to_uppercase()
    |> item in println!("{}", item)
);
```

You can of course accept a pattern in this "weird closure".
This example extracts the inner `bool` value from the
`Test` instance in the pipeline with pattern matching.

```rust
struct Test(bool);
let result = pipe!(Test(true) |> Test(it) in it);
assert_eq!(result, true);
```
