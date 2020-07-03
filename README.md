# Nikel-rs

[![](https://img.shields.io/crates/d/nikel-rs)](https://crates.io/crates/nikel-rs)
[![Crates.io](https://img.shields.io/crates/v/nikel-rs)](https://crates.io/crates/nikel-rs)
[![Crates.io](https://img.shields.io/crates/l/nikel-rs)](https://crates.io/crates/nikel-rs)

A Rust library for interacting with the UofT API Nikel: http://nikel.ml

# Usage

```rust
use std::vec:Vec;
use nikel_rs::NikelAPI;

let client = NikelAPI::Client::new(); // Create client
let opts: Vec<(&str, &str)> = vec![("code", "CSC108"), ("campus", "mississauga")] // Query options
let resp = client.courses(opts).expect("Error!"); // Get
if resp.status_code == 200 {
  println!("{}", resp.response[0].description); // Print the first course's description
} else {
  println!("Error!");
}
```

See [Nikel-CLI](https://github.com/George-lewis/Nikel-CLI) for a more complete example
