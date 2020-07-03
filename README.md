# Nikel-rs

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
