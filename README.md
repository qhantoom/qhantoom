# qhantoom

> *a fresh wind to design safe and optimised applications*

<p align="center">
  <img src="./src/misc/qhantoom-banner.png">
</p>

## About

under development | under development | under development

[design](./src/doc/design.md) - [roadmap](./src/doc/roadmap.md) — [syntax](./src/doc/syntax.md)

## Usage

```c
fun fib(n: uint): [s32] {
  mut x = [1, 1];

  for i := 2..n -> x.push(x[i - 1] + x[i - 2]);

  x
}

fun main() {
  #print("{:?}", fib(42));
}
```

## Development

[Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) must be installed on your machine before.

**start**

| run	    | desc                    | cmd                         |
|:--------|:------------------------|:----------------------------|
| compile	| run the `aot` compiler  | `cargo run compile <file>`  |
| repl	  | run the `jit` compiler  | `cargo run repl`            |

## License

[MIT](./LICENSE)
