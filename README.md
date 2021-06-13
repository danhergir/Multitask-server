# Overview

This is multitasking server built with Rust with different modules, allowing the use of threads and serving HTML in the browser.

The purpose of this project is to learn more about Rust and the special things this programming language can do compared to others. 

Rust aims to be a language with very good support for concurrency and flexible parallelism, allowing powerful APIs without losing any guarantee of thread safety (or memory safety).

[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

- Rust
- VSCode

# Useful Websites
* [Rust Course Platzi](https://platzi.com/clases/servidores-rust/)
* [Rust Official Site](https://www.rust-lang.org/learn)

# Future Work
* Create an user interface
* Serve HTML and views in the server
* Use a framework


***Run the server:***

	  $ cargo run --release --bin server

	  $ cargo run --release --bin client_synchronous
	  $ cargo run --release --bin client_async
	  $ cargo run --release --bin client_synchronous_parallel
