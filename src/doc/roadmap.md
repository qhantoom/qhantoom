# Roadmap

**compiler**

* [ ] front-end
  * [x] `tokenizer`
  * [x] `parser`
  * [ ] `analyzer`

* [ ] middle-end | *needs help*

* [ ] back-end | *needs help*
  * [ ] cranelift
  * [ ] `codegen` | *wip*
  * [x] emit native object `.o` file
  * [x] create an executable

* [ ] comments
  * [x] line comments
  * [ ] block comments
  * [x] doc line comments
  * [ ] doc block comments

* [ ] types
  * [ ] `bool`
  * [ ] `s8`, `s16`, `s32`, `s64`, `sint`
  * [ ] `u8`, `u16`, `u32`, `u64`, `uint`
  * [ ] `char`
  * [ ] `str`

* [ ] operators
  * [x] `+`, `-`, `*`, `/`, `%`
  * [x] `==`, `!=`, `<`, `<=`, `>`, `>=`
  * [x] `=`
  * [x] `+=`, `-=`, `*=`, `/=`
  * [x] `|=`, `&=`,
  * [x] `|`, `&`
  * [x] `||`, `&&`
  * [ ] `<<`, `>>`

* [ ] booleans
  * [ ] `true`
  * [ ] `false`

* [ ] numbers
  * [x] `integers`
  * [x] `floats`
  * [x] `hexadecimals`

* [ ] variables
  * [ ] `imu`
  * [x] `val`
  * [x] `mut`
  * [ ] multiples

* [ ] functions
  * [ ] functions
  * [ ] closures
  * [ ] higher order functions
  * [ ] `return`
  * [ ] calls

* [ ] ffi
  * [ ] `exp`
  * [ ] `ext`
  * [ ] `mod`

* [ ] macros
  * [ ] macros
  * [ ] calls

* [ ] assertions
  * [ ] `unit`
  * [ ] `mock`
  * [ ] `test`

* [ ] branches
  * [x] `if`
  * [ ] `else if`
  * [x] `else

* [ ] pattern matching
  * [ ] `match`

* [ ] loops
  * [x] `loop`
  * [ ] `for`
  * [x] `while`
  * [x] `break`
  * [x] `continue`

* [ ] data structures
  * [ ] `action`
  * [ ] `array`
  * [ ] `enum`
  * [ ] `hash`
  * [ ] `struct`
  * [ ] `tuple`

* [ ] modules
  * [ ] `load`
  * [ ] `bind`

* [ ] library
  * [ ] `std`
  * [ ] `ogl`
  * [ ] `wgl`

**optimizations**

* [ ] exclude unused functions
* [ ] function call inlining
* [ ] expression optimization

**linter**

* [ ] unused variables
* [ ] unused arguments
* [ ] unused modules

**tools**

* [ ] `repl` | *wip*
* [ ] extensions | `vscode`
* [ ] package manager
* [ ] [qhantoom.dev](https://qhantoom.dev) | *wip*
