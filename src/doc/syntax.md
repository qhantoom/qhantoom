# qhantoom compiler: syntax

defining a relevant syntax that makes semantically meaningful is not easy. many programming language designers neglect these aspects completely. in general, a syntax should be simple, consistent, intuitive and flexible (a bit like legos).

my choices in terms of syntax focus on the perfectible points detected in certain programming languages. these choices only concern me and in no way represent the procedure to follow.

inspirations: [elm](https://en.wikipedia.org/wiki/Elm_(programming_language)), [erlang](https://en.wikipedia.org/wiki/Erlang_(programming_language)), [fjölnir](https://en.wikipedia.org/wiki/Fj%C3%B6lnir_(programming_language)), [haskell](https://en.wikipedia.org/wiki/Haskell_(programming_language)), [hermes](https://en.wikipedia.org/wiki/Hermes_(programming_language)) [icon](https://en.wikipedia.org/wiki/Icon_(programming_language)), [jai](https://en.wikipedia.org/wiki/Draft:Jai_(programming_language)), [planc](https://en.wikipedia.org/wiki/PLANC), [rust](https://en.wikipedia.org/wiki/Rust_(programming_language))

## Tokens

**hello world**

```
fun main := () {
  print("hello, world!👽");
}
```

**tokenization**

```
fun
main
:
=
(
)
{
print
(
"hello, world!👽"
)
;
}
```

## Comments

`#` - comment line symbol    

`#+` - comment block scope start symbol   
`#-` - comment block scope end symbol  

`#!+` - comment block doc scope start symbol    
`#!` - comment block doc symbol    
`#!-` - comment block doc scope end symbol    

```
# this is a line comment

#+
this is a block comment
#-

#!+
#! this is a block doc comment 
#!-
```

## Identifiers
    
an identifier cannot start with a number

```
square    cosinus   degrees
_tmp      add_tmp   to_tmp_
vector1   vector2   vector3 
```

## Keywords

```
as      async       await   bench   break   capsule
chan    continue    else    enum    exp     ext
false   fun         for     if      load    loop
match   mock        mod     mut     pack    pub
ref     return      Self    self    set     spawn
struct  test        type    unit    val     imu
wasm    where       while   _
```

## Data Types

`boolean` - represents two constants such as `true`, `false`    
`numeric` - represents arithmetic types   
`string` - represents a string slice    
`derived` - represents derives types    

**type keywords**

```
u8    u16   u32   u64   uint
s8    s16   s32   s64   sint
f32   f64   Fun 
bool  char  str
```

**type operators**

```
! () [] {} \->
```

**type**

```
type Symbol = s32;
```

## Bindings

`imu` - immutable constant keyword    
`val` - immutable local keyword   
`mut` - mutable local keyword   

**constants**

```
imu SQRT: s8 = 0;
imu SQRT := 0;
```

**immutables**

```
val x: s8 = 0;
val x := 0;
```

**mutables**

```
mut y: s8 = 0;
mut y := 1;
```

**inferred types**

```
val a := true;
val b := false;
val c := a == b;
```

**multiple assignments**

```
val a, b, c: s8 = 0;
val a, b, c := 0;
```

## Operators

```
+   -   *   /   %   !   .   :   &   <   >   =   |   
++  --  **  //  %%  !!  ..  ::  &&  <<  >>  ==  ||  
+=  -=  *=  /=  %=  !=  .=  :=  &=  <=  >=
?   \   ->  =>  
```

## Branches

**if**

```
if true { a }
```

**if else**

```
if true { a } else { b }
```

**if else if**

```
if a < b {
  0
} else if a > b {
  1
} else {
  panic()
}
```

## Loops

**loop**

```
loop {}
```

**for**

```
for nums = (num) {}
for 0.=4 = (num) {}
```

**while**

```
while true {}
while 0 < 1 {}
```

## Functions & Closures

`fun` - function declaration keyword    
`\->` - arrow function symbol   

```
# function declaration
fun sqrt: Fun(u8) -> u8 = () { x * x }

# closure expression
val sqrt: Fun(u8) -> u8 = () { x * x }

# closure expression
mut sqrt: Fun(u8) -> u8 = () { x * x }

# arrow function expression
mut sqrt: \ u8 -> u8 = \ n -> x * x;
```

## Arrays

```
[10, 23, 12, 3]
```

## Structures

```
struct Button {
  id: uint,
  name: str,
}

set Button {
  fun new: Fun(str) -> Button = (name) {
    Button {
      id = 0,
      name = name,
    }
  }
}

val button: Button = Button {
  id = 0,
  name = "button-name",
};

val button: Button = new::Button("button-name");
```

## Recursions

**factorial**

```
fun factorial: Fun(uint) -> uint = (i) {
  if i <= 1 {
    return 1;
  }

  i * factorial(i - 1)
}
```

## Capsules

`capsule` - represents an interface   

```
capsule Vec2 {
  fun mul: Fun(uint) -> (uint);
}

struct Point {
  x: f32,
  x: f32,
}

set Vec2 for Point {
  fun mul: Fun(uint) -> (uint) = () {}
}
```

## Modules

`bind` - set module keyword   
`pack` - set module keyword   
`load` - get module keyword   

**bind | pack**

```
pub bind ident;
pub bind ident;
bind ident;
```

or

```
pub pack ident;
pub pack ident;
pack ident;
```

**load**

```
load std::ident;
load lib::ident::ident;
load lib::ident::ident::(a, b);
```

## Matching

```
match op {
  '+' => add(x, y),
  '-' => sub(x, y),
  '*' => mul(x, y),
  '/' => div(x, y),
  _ => panic("with msg") # maybe a panic keyword instead?
}
```

## Attributes

```
|> repr: c
|> derive: clone, debug.
```

## Enums

```
enum MyEnum {
  BasicEnum,
  TupleEnum(),
  StructEnum { x: uint },
}
```

## Asynchronous

`async` - make asynchronous keyword    
`await` - make thread keyword   

```
async {
  await 0
}
```

## Collections

```
val user: Hash<str, str> = .{
  "username" = "",
  "password" = "",
};
```

## Silents

```
mut x = --;
```

## Channels

`chan` - make channel keyword   
`spawn` - make thread keyword   

```
chan tx rx = --;
spawn tx.send(1);
print("received: {}", rx.on());
```

## Assertions

`unit` - assertion unit keyword   
`mock` - assertion mock keyword   
`test` - assertion test keyword   

**unit**

```
unit {
  # add mocks and tests
}
```

**mock**

```
mock tokenization_mock = () {
  # mock computation
}
```

**test**

```
test tokenization_test = () {
  # test computation
}
```

## Benchmarks

`bench` - bench computation keyword   

**bench**

```
bench tokenization_benchmark = () {
  # bench computation
}
```

## FFI

`ext` - call c function keyword   
`exp` - call javascript function keyword    
`mod` - call rust function keyword    

**c**

```
ext fun cos: Fun(uint) -> (uint);
```

**javascript**

```
exp fun cos: Fun(uint) -> (uint);
```

**rust**

```
mod fun cos: Fun(uint) -> uint;
```

## WebAssembly

`wasm` - represents webassembly keyword   

**wasm**

```
wasm fun main := () {
  # something to do
}
```
