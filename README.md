# qhantoom

> *a fresh wind to design safe and optimised applications*

## About

under development | under development | under development

[roadmap](./src/doc/roadmap.md) — [syntax](./src/doc/syntax.md)

## Usage

```ocaml
fun main := () {
  fun fibonacci : (uint) -> uint = (n) {
    if n == 0 || n == 1 {
      return n;
    }

    let f := [0, 1];

    for 2..n = (i) {
      f[i] = f[i - 1] + f[i - 2];
    }

    return f[n];
  }

  #print("{}", fibonacci(7));
}
```

## Goals

* no gc
* type system
* high performance (fast compilation time)
* backend (`aot` and `jit`) | `cranelift`
* small binaries size

## Development

[Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) must be installed on your machine before.

**clone**

```
$ git clone https://github.com/qhantoom/qhantoom.git
```

**build**

```
$ cargo build --release
```

**start**

| run	    | desc                    | cmd                            |
|---------|-------------------------|--------------------------------|
| compile	| run the `aot` compiler  | `cargo run compile <filename>` |
| repl	  | run the `jit` compiler  | `cargo run repl`               |

## License

[MIT](./LICENSE)
