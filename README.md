# qhantoom

> *a programming language in progress..*

## About

a language that has a taste for designing secure applications.

## Usage

**fibonacci**

```
fun main := {
  fun fibonacci: Fun(uint) -> Vec<uint> = (n) {
    mut x1 := [1, 1];

    for 2.=n = i {
      val x2 := x1[i - 1] + x1[i - 2];
      x1.push(x2);
    }

    x1
  }

  print(fibonacci(7));
}
```

## Goals

* no gc
* type system
* high performance
* syntax from saturn
* small binaries size
* fast compilation time
* backend | `cranelift`

## Language Design

about the `roadmap`, checkout this page [here](doc/roadmap.md)      
about the `syntax`, checkout this page [here](doc/syntax.md)      
<!-- qhantoom lab == online repl -->

## Development

[Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) must be installed on your machine before.

**bin**

| run     | cmd                                             |
|---------|-------------------------------------------------|
| clone   | `git clone https://github.com/qurity/qoeur.git` |
| build    | `cd qhantoom && cargo build --release`         |

**start**

| run     | cmd                                         |
|---------|---------------------------------------------|
| compile | `cargo run compile <filename>`              |
| repl    | `cargo run repl`                            |
