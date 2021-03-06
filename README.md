#chilli (A Sharp Pencil fork)

[![Build Status](https://travis-ci.org/armersong/chilli.svg?branch=master)](https://travis-ci.org/armersong/chilli) [![Crates.io Version](https://img.shields.io/crates/v/chilli.svg)](https://crates.io/crates/chilli/)

A microframework for Rust inspired by Flask.

```rust
extern crate chilli;

use chilli::{Pencil, Request, Response, PencilResult};

fn hello(_: &mut Request) -> PencilResult {
    Ok(Response::from("Hello World!"))
}

fn main() {
    let mut app = Pencil::new("/web/hello");
    app.get("/", "hello", hello);
    app.run("127.0.0.1:5000");
}
```

One simple guide: 
https://github.com/golddranks/sharp_pencil
https://fengsp.github.io/blog/2016/3/introducing-pencil/

If you feel anything wrong, feedbacks or pull requests are welcome.
