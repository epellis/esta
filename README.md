# Esta

[![CircleCI](https://circleci.com/gh/epellis/esta.svg?style=shield)](https://circleci.com/gh/epellis/esta)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/epellis/esta.svg?style=popout-square)


Esta is a gradually typed, interpreted language and virtual machine implementation of my own design written in Rust

_Interpreted_: `.est` source code is compiled into byte code (simple assembly instructions)
                and run on the Esta VM.

_Gradually Typed_: The Esta Interpreter can infer variable type (e.g.):
```c
var a: num = 4;     // Explicitly declare a is an int

var c = a + b;      // Since c and b have unknown types, they adopt a's type
```

## Syntax

Esta's syntax is an LR(1) grammar that takes most of it's ideas from
the C-family of languages. It is most similar to JavaScript and Go.
Here is an example snippet of code:
```c
fun multiply(a, b) {
    if a <= 0 {
        return b;
    } else {
        return b + multiply(a - 1, b);
    }
}

print(multiply(3, 4) == 12);
```

## Blog Posts
- [Writing and traversing an AST in Rust](http://nedellis.com/2019/05/08/esta_1/)

## Development

You are welcome to check the project out and try some of the demos provided.
```
git clone https://github.com/epellis/esta.git
cd esta
cargo build
RUST_LOG=esta=debug cargo run demos/hello.est
```

You can also measure performance by evaluating a recursive
Fibonacci Sequence O(n^2) calculation.
```
cargo bench
```

## Deployment

_WIP_: Installer coming soon

```
cargo run --release my_program.est
```
