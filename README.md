# Hash Consing Library
[![crates.io](https://img.shields.io/crates/v/hash_cons.svg)](https://crates.io/crates/hash_cons)
[![docs.rs](https://docs.rs/hash_cons/badge.svg)](https://docs.rs/hash_cons)

The Hash Consing Library (`hash_cons`) is a Rust library that implements efficient hash consing techniques, 
making it a robust choice for both single-threaded and multi-threaded Rust applications. 
At its core, the library is designed to focus on type-safety, efficiency, and zero-cost abstractions.

## Efficiency and Idiomatic Rust

This library is designed to be inherently idiomatic to Rust, ensuring efficient memory management and optimal performance. It utilizes Rust's unique ownership and borrowing rules to manage immutable data structures. The key feature is its automatic memory management: when a value is no longer referred to anywhere in your program, the library automatically deallocates it, preventing memory leaks and optimizing resource usage. This makes it an excellent tool for applications where duplicate data structures are common, ensuring  memory efficiency.

## Features

* **Single-Threaded (Default Feature)**: Tailored for applications not requiring thread safety, this default mode uses `Rc` and `RefCell` for efficient memory management, avoiding the overhead of synchronization mechanisms.
* **Thread-Safe**: Designed for multi-threaded applications, it uses `Arc` and `RwLock` to guarantee safe concurrent access, aligning with Rust's emphasis on safety.

## Usage

To integrate `hash_cons` into your project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
hash_cons = "0.1.1"  # Replace with the actual version
```

By default, the library operates in a single-threaded environment. For multi-threaded support, enable the `thread-safe` feature:

```toml
# Default single-threaded
hash_cons = "0.1.1"

# For multi-threaded environments
hash_cons = { version = "0.1.1", features = ["thread-safe"] }
```

## Examples

### Single-Threaded Usage

```rust
use hash_cons::{HcTable, Hc};

#[derive(Hash, PartialEq, Eq)]
enum BoolExpr {
    Const(bool),
    And(Hc<BoolExpr>, Hc<BoolExpr>),
    Or(Hc<BoolExpr>, Hc<BoolExpr>),
    Not(Hc<BoolExpr>),
}

fn main() { 
    let table: HcTable<BoolExpr> = HcTable::new();
    let const_true = BoolExpr::Const(true);
    let hc_true: Hc<BoolExpr> = table.hashcons(const_true); // hc_true is automatically dropped when no longer used
}
```

### Thread-Safe Usage

```rust
use hash_cons::{AhcTable, Ahc};
use std::thread;

#[derive(Hash, PartialEq, Eq)]
enum BoolExpr { 
    Const(bool),
    And(Ahc<BoolExpr>, Ahc<BoolExpr>),
    Or(Ahc<BoolExpr>, Ahc<BoolExpr>),
    Not(Ahc<BoolExpr>),
}

fn main() { 
    let table: AhcTable<BoolExpr> = AhcTable::new();
    let thread_handle_ahc_false = thread::spawn(move || {
        table.hashcons(BoolExpr::Const(false))
    });
    let ahc_false: Ahc<BoolExpr> = thread_handle_ahc_false.join().unwrap(); // Safe for concurrent use across threads 
}
```

## Contributing

We welcome contributions and suggestions to make `hash_cons` better. If you have ideas or improvements, feel free to submit a pull request or open an issue in the [repository](https://github.com/karan9123/hash_cons).


## License

Licensed under the MIT License. See the [LICENSE](https://github.com/karan9123/hash_cons/LICENSE) file for more details.
