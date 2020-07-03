# Nikel-rs

[![](https://img.shields.io/crates/d/nikel-rs)](https://crates.io/crates/nikel-rs)
[![Crates.io](https://img.shields.io/crates/v/nikel-rs)](https://crates.io/crates/nikel-rs)
[![Crates.io](https://img.shields.io/crates/l/nikel-rs)](https://crates.io/crates/nikel-rs)

A Rust library for interacting with the UofT API Nikel: http://nikel.ml

# Usage

```rust
use std::collections
use nikel_rs::NikelAPI;
use nikel_rs::*;

let client = NikelAPI::Client::new(); // Create client
let opts: HashMap<&str, &str> = [("code", "CSC108"),
                                ("campus", "mississauga")]
                                 .iter()
                                 .clone()
                                 .collect(); // Query options
let resp = client.courses(opts).unwrap(); // Get
if resp.status_code == 200 {
  println!("{}", resp.response[0].description); // Print course description
} else {
  println!("Error!");
}
```

See [Nikel-CLI](https://github.com/George-lewis/Nikel-CLI) for a more complete example
