# MoabDB Official Rust Library
MoabDB loves Rust! This library makes it super easy to use MoabDB in your Rust projects.
It's a simple wrapper around the MoabDB HTTP API.

## Usage
Add this to your Cargo.toml:
```toml
[dependencies]
moabdb = "*"
```

Create a time window to search for data:
```rust
let window = WindowBuilder::new()
    .length(Years(1))
    .build()
    .expect("Failed to create time window");
```

Send a query to MoabDB:
```rust
let df = moabdb::get_equity("AAPL", window, false, None).expect("Failed to get data");
```

## Design
The design of the library is targeted at being identical to the Python library. This is so
people can easily switch between the two languages without having to learn a new API.

## License
This library is licensed under the MIT license. See the LICENSE file for more information.