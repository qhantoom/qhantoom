# qhantoom

> *a fresh wind to design safe and optimised applications*

<p align="center">
  <img src="./src/doc/qhantoom-banner.png">
</p>

## About

under development | under development | under development

[design](./src/doc/design.md) - [roadmap](./src/doc/roadmap.md) — [syntax](./src/doc/syntax.md)

## Usage

```q
load std::math::(min, max);

ext cos(x : uint) : uint;
ext abs(x : uint) : uint;

fun main() {
  val min : uint = min(1, 3);
  val max := max(1, 3);

  #print("minimum: {min}, maximum: {max}");

  mut cos : uint = cos(1);
  mut abs := abs(1);

  #print("cosine: {min}, absolute: {abs}");
}
```

## Goals

* no gc
* hybrid type system
* high performance (fast compilation time)
* backend (`aot` and `jit`) | `cranelift`
* small binaries size

## Development

[Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) must be installed on your machine before.

**clone**

```
git clone https://github.com/qhantoom/qhantoom.git
```

**build**

```
cargo build --release
```

**start**

| run	    | desc                    | cmd                         |
|:--------|:------------------------|:----------------------------|
| compile	| run the `aot` compiler  | `cargo run compile <file>`  |
| repl	  | run the `jit` compiler  | `cargo run repl`            |

**repl**

the jit compiler execute the source code and then output the result.

**compile**

the aot compiler produce an object file named `test.o`. when the compilation is done you have to follow those commands to ensure that your program produces the right result:

```
gcc -o test test.o
```

```
./test
```

```
echo $?
```

the last command will print the result to the stdout.

## License

[MIT](./LICENSE)
