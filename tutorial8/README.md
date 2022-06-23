# Systems Programming

Sometimes in Rust we want really low-level control over how our program runs.
Examples:

- Manipulating and passing around direct references to memory (pointers);

- Making system calls;

- Interfacing with C code;

- Dealing with low-level IO (e.g. file handles and pipes; network hosts, ports, and sockets); and

- Spawning and handling threads and processes.

This tutorial will be split into multiple parts.

## Part 1

- Unsafe code (`unsafe` blocks and functions)

- Introduction to smart pointer types: `IDManger` case study

## Part 2

- C code wrappers and system calls (`libc`, `nix`)

- Smart pointer types continued (`Box`, `Cell`, `Rc`, `RefCell`, etc.)

## Additional material

- Manual memory management (the `Drop` trait, `mem::replace`)

- HW3 common mistakes
