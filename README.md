# monkey-language

## Overview
monkey-language is a statically-typed, compiled programming language and compiler written in Rust. Initially created as a learning project for compiler construction, it is actively developed with the goal of reaching a stable 1.0 version that is practically usable.

## Goals and Motivation
The project aims to provide a platform for exploring compiler design and programming language implementation. While it is currently a hobby project, the long-term goal is to create a stable and functional language.

## Key Language Features
- **Statically typed** with type inference.
- **Compiled language** for performance.
- **Immutable variables by default**: Mutability must be explicitly declared using `mut` (similar to Rust).
- **Explicit mutability** applies to both variable declarations and function/method parameters.

## Build Instructions
1. Ensure you have [Rust](https://www.rust-lang.org/) installed.
2. Clone the repository:
   ```bash
   git clone https://github.com/tomyk9991/monkey-language/
   cd monkey-language
   ```
3. Build the project using Cargo:
   ```bash
   cargo build --release
   ```

## CLI Usage
The compiler provides a command-line interface. Typical usage:
```bash
cargo run -- <options>
```
### Options
- **Input source file**: Specify the `.monkey` file to compile (default: current directory).
- **Target OS**: Choose the target OS (`Linux`, `Windows`, `WSL`).
- **Build-only mode**: Compile without running the program.
- **Scope printing**: Enable production/debug scope printing.
- **Optimization level**: Set optimization level (default: `O1`).

### Example
```bash
cargo run -- --input monkey-language-project/main.monkey --target-os windows -o0 --print-scope Production
```

## Language Examples
### Variables and Mutability
```monkey
let x = 10; // Immutable by default
let mut y = 20; // Explicitly mutable
y = y + x;
```

### Simple Function
```monkey
fn add(a: i32, b: i32): i32 {
    return a + b;
}

let result = add(5, 10);
```

## Project Status
monkey-language is under active development but is not yet production-ready. The primary development platform is Windows, with early groundwork for Linux support.

### Roadmap
- Stabilize core language features.
- Expand platform support.
- Improve optimization and code generation.

## License
This project currently has no defined license.
