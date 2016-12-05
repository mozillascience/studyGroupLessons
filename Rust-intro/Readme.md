# Rust-intro
Introduction to Rust programming language

## Overview

"Rust is a systems programming language that runs blazingly fast, prevents
nearly all segfaults, and guarantees thread safety." &ndash;
[rust-lang.org](https://www.rust-lang.org/)

### What _is_ Rust? 

Rust is:
- Fast
- Safe
- Functional
- Zero-cost

### Fast

- Rust compiles to native code
- Rust has no garbage collector
- Most abstractions have zero cost
- Fine-grained control over lots of things
- Pay for exactly what you need...
- ...and pay for most of it at compile time

### Safe

- No null
- No uninitialized memory
- No dangling pointers
- No double free errors
- No manual memory management!

### Functional

- First-class functions
- Trait-based generics
- Algebraic datatypes
- Pattern matching

### Zero-Cost 100% Safe Abstractions

- Rust's defining feature
- Strict compile-time checks remove need for runtime
- Big concept: Ownership

### Development

- Rust is led by the Rust Team, mostly at Mozilla Research.
- Very active community involvement - on GitHub, Reddit, irc.
    - [Rust Source](https://github.com/rust-lang/rust/)
    - [Rust Internals Forum](https://internals.rust-lang.org/)
    - [/r/rust](http://www.reddit.com/r/rust)

### Big Rust Projects

- [Servo](https://github.com/servo/servo)
- [Piston](https://github.com/PistonDevelopers/piston)
- [MIO](https://github.com/carllerche/mio)
- [nickel.rs](http://nickel.rs/)
- [iron](https://github.com/iron/iron)
- [lalrpop](https://github.com/nikomatsakis/lalrpop)
- [cargo](https://github.com/rust-lang/cargo)
- [Rust itself!](https://github.com/rust-lang/rust/)

## Rust Installation

~~~~
curl -sSf https://static.rust-lang.org/rustup.sh | sh
~~~~

## To check the version

~~~~
rustc --verison
~~~~

## To uninstall rust 

~~~~
sudo /usr/local/lib/rustlib/uninstall.sh
~~~~

## Running the first hello world problem

* basic code
~~~~
fn main() {
    println!("Hello, world!");
}
~~~~

* How to compile and run
~~~~
$ rustc main.rs
$ ./main
Hello, world!
~~~~

## Cargo - Rust’s build system and package manager

* check if Cargo is installed

~~~~
cargo --version
~~~~

* Cargo expects your code to be inside src dir
* Cargo.toml is a configuration file

* Building and running a Cargo project

~~~~
$ cargo build
   Compiling hello_world v0.0.1 (file:///home/yourname/projects/hello_world)
$ ./target/debug/hello_world
Hello, world!

//or

$ cargo run
     Running `target/debug/hello_world`
Hello, world!
~~~~

* Cargo.lock - tracks the dependencies of the application

* Makes a complete Cargo project to hack on
~~~~
cargo new hello_world --bin
~~~~

## Rust Components
* [Assignment](./code_samples/assigning.rs)
* [String](./code_samples/string.rs)
* [Condition](./code_samples/condition.rs)
* [Looping](./code_samples/looping.rs)
* [Array](./code_samples/array.rs)
* [Vector](./code_samples/vector.rs)
* [Tuples](./code_samples/tuples.rs)
* [Function](./code_samples/functions.rs)
* [Closure](./code_samples/closures.rs)
* [Pointer](./code_samples/pointer.rs)
* [Struct](./code_samples/struct.rs)
* [Trait](./code_samples/trait.rs)
* [Enum](./code_samples/enum.rs)

### To run the code 
~~~~
rustc ./code_samples/code_name.rs -A warnings
./code_name
~~~~

#Reference link

* [Rust Book](https://doc.rust-lang.org/book)
* [Rust Video tutorial](https://www.youtube.com/watch?v=U1EFgCNLDB8)

