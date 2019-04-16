# Esta

Esta is a gradually typed, interpreted language of my own design written in Rust

_Interpreted_: `.est` source code is compiled into byte code (simple assembly instructions)
                and run on the Esta VM.

_Gradually Typed_: The Esta Interpreter can infer variable type (e.g.):
```c
var a: num = 4;     // Explicitly declare a is an int

var c = a + b;      // Since c and b have unknown types, they adopt a's type
```

## Syntax

Esta's syntax is an LR(1) grammar that takes most of it's ideas from
the C-family of languages. Here is an example snippet of code:
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

## Development

You are welcome to check the project out and try some of the demos provided.
```
git clone https://github.com/epellis/esta.git
cd esta
cargo build
cargo run demos/hello.est
```

## Deployment

_WIP_: Installer coming soon

```
cargo build --release
cargo run my_program.est
```
