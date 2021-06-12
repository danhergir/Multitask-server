Concurrent programming, where different parts of a program run independently, and parallel programming, where different parts of a program run at the same time, are becoming increasingly important as more computers take advantage of their multiple processors.

Rust aims to be a language with very good support for concurrency and flexible parallelism, allowing powerful APIs without losing any guarantee of thread safety (or memory safety).


$ cargo run --release --bin server

$ cargo run --release --bin client_synchronous
$ cargo run --release --bin client_async
$ cargo run --release --bin client_synchronous_parallel
