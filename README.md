# Rust Playground

🦀 Learn Rust by examples.

This repository contains a set of tutorials to help you learn Rust fast.

If you like this project, please consider giving it a star ⭐

Happy learning! 😃

## Table of Contents

- [Table of Contents](#table-of-contents)
- [What is Rust and why should I learn it?](#what-is-rust-and-why-should-i-learn-it)
- [What is Cargo and how to use it?](#what-is-cargo-and-how-to-use-it)


## What is Rust and why should I learn it?

Rust is a systems programming language designed for performance, safety, and concurrency.

It was created from the ground up with a focus on preventing common errors errors like null pointer dereferencing, data races, and buffer overflows to happen.

### What are the use-cases for Rust?

- Operating Systems
- Device Drivers
- Web Servers
- Scientific Computing
- Gaming

### What are the highlights of Rust?

- Safe memory management without garbage collection

- Strong support for concurrency and parallelism

- High performance

- Expressiveness and powerful abstractions

- Built-in cross platform support

- Growing community and ecosystem

## What is Cargo and how to use it?

Cargo is the package manager and build system for Rust. It is an integral part of the Rust programming language ecosystem.

### Create a Project

```Bash
cargo new --bin <project name> # Create a new binary project.
cargo new --lib <project name> # Create a new library project.
```


> Manage it with git!

### Building

```Bash
cargo build            # Build in debug mode.
cargo build --release  # Build in release mode.
```

### Running

```Bash
cargo run            # Build and run in debug mode.
cargo run --release  # Build and run in release mode.
```

### Testing

```Bash
cargo test            # Run tests in debug mode.
cargo test --release  # Run tests in release mode.
```

### Formatting and Linting

```Bash
cargo fmt     # Format the code.
cargo clippy  # Lint the code.
```
