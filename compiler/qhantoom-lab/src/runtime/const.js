export const HOWTO = Object.freeze({
  start: 'fun main := () {\n\t\n}',
  binary: 'fun main := () {\n\t3 + 39;\n}',
  binding: 'fun main := () {\n\tval x: s8 = 3 + 39;\n}',
  variable: 'imu SQRT: s8 = 0;\n\nfun main := () {\n\tval x: s8 = 1_000;\n\tmut y: s8 = 1_000_000;\n\tval x y z: s8 = 0;\n}',
  function: `
fun main := () {
  val sqrt: (u8) -> u8 = () { x * x };
  mut sqrt_mut: \\ u8 -> u8 = n -> x * x;

  sqrt(9);
  sqrt_mut(9);
}

fun srqt: (s8) -> s8 = (x) {
  x * x
}
  `,
  if: 'fun main := () {\n\tif true { 1 } else { 0 }\n}',
  array: 'fun main := () {\n\tval a : [] = [1, 2, 3];\n\ta[2];\n}',
  hash: 'fun main := () {\n\tval a : hash<str, str> = .{\n\t\t"firstname": "john",\n\t\t"lastname": "doe"\n\t};\n\n\ta["firstname"];\n}',
  unary: 'fun main := () {\n\tval a : bool = !true;\n\tval b : bool = !0;\n\ta == b;\n}',
  int: 'fun main := () {\n\tval a : int = 2;\n\tval b : int = 2;\n\tval c : int = a + b;\n\tc;\n}',
  bigint: 'fun main := () {\n\tval a : int = 1_000_000_000;\n\tval b : int = 1_000_000;\n\tval c : int = a + b;\n\tc;\n}',
  float: 'fun main := () {\n\tval a : int = 1.234;\n\tval b : int = 1e4;\n\tval c : int = a + b;\n\tc;\n}',
  str: 'fun main := () {\n\tval a : str = "abc";\n\tval b : str = "def";\n\tval c : str = a + b;\n\tc;\n}',
  loop: 'loop {\n\n}',
  for: 'for 0.=4 = (num) {\n\n}',
  while: 'while true {\n\n}',
  silent: `
fun main := () {
  mut x = --;
}
  `,
  load: `
load std::ident;
load lib::ident::ident;
load lib::ident::ident::(a, b);
  `,
  async: `
fun main := () {
  async {
    await 0
  }
}
  `,
  channel: `
fun main := () {
  chan tx rx = --;
  spawn tx.send(1);

  print(rx.on());
}
  `,
  matching: `
fun main := () {
  calculator(3, 39, Binop::Add);
}

enum Binop {
  Add,
  Sub,
  Mul,
  Div,
}

fun add: (uint, uint) -> uint = (x, y) { x + y }
fun sub: (uint, uint) -> uint = (x, y) { x - y }
fun mul: (uint, uint) -> uint = (x, y) { x * y }
fun div: (uint, uint) -> uint = (x, y) { x / y }

fun calculator: (uint, uint, Binop) -> uint = (lhs, rhs, op) {
  match op {
    Binop::Add => add(x, y),
    Binop::Sub => sub(x, y),
    Binop::Mul => mul(x, y),
    Binop::Div => div(x, y),
  }
}
  `,
  capsule: `
capsule Vec2 {
  fun mul: (uint) -> (uint);
}

struct Point {
  x: f32,
  x: f32,
}

set Vec2 for Point {
  fun mul: (uint) -> (uint) = () {}
}
  `,
  struct: `
fun main := {
  val btn: Button = Button {
    id = 0,
  };
}

struct Button {
  id: uint,
}

set Button {
  fun new: (uint) -> Self = (id) {
    Self {
      id = id,
    }
  }
}
  `,
  test: `
unit {
  mock tokenization_mock = () {

  }
  
  test tokenization_test = () {
    must!(4 eq 4);
  }
}
  `,
  benchmark: `
bench tokenization_benchmark = () {

}
  `,
  ffi: `
ext fun cos: (uint) -> uint;
exp fun cos: (uint) -> uint;
mod fun cos: (uint) -> uint;
  `,
  wasm: `
wasm fun main := () {

}
  `,
  fibonacci: `
fun main := {
  fun fibonacci: Vec<uint> = (n) {
    mut x1 := [1, 1];

    for 2.=n = i {
      val x2 := x1[i - 1] + x1[i - 2];
      x1.push(x2);
    }

    x1
  }

  print(fibonacci(7));
}
  `,
  factorial: `
fun main := {
  factorial(3);
}

fun factorial: (uint) -> uint = (i) {
  if i <= 1 {
    return 1;
  }

  i * factorial(i - 1)
}
  `,
  web: `
<script>
  val x = 10;
</script> 

<h1>x = {x}!</h1>
  `,
});

export const DEFAULT_IFRAME_VIEW = Object.freeze({
  iframe: `
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Document</title>
        <style>
          *,
          *:after,
          *:before {
            box-sizing: border-box;
            -webkit-font-smoothing: antialiased;
            -moz-osx-font-smoothing: grayscale;
          }
          html,
          body {
            width: 100vw;
            height: 100vh;
            overflow: hidden;
            background: white;
          }
          body {
            position: relative;
          }
        </style>
      </head>
      <body>
          <h1>hello, world</h1>
      </body>
    </html>
  `,
});

export const DEFAULT_EDITOR_OPTION = Object.freeze({
  lineNumbers: true,
  mode: "qhantoom",
  theme: "monokai",
  indentWithTabs: true,
  tabSize: 2,
  viewportMargin: Infinity,
});
