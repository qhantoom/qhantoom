# Syntax

## Comments

`--` comment line symbol    

`-!` doc comment line

`-%` - comment block scope start symbol   
`%-` - comment block scope end symbol   

`-%!` - comment block doc scope start symbol   
`%-` - comment block doc symbol   

```
-- this is a line comments
-! this is a doc line comments
-% this is a block comments %-
-! this is a doc comments !-
```

## Operators

```
+   -   *   /   %   !   =   &   |   <   >   .   :   ;   \   ?   @   _
                        ==  &&  ||  <<  >>  ..  ::
+=  -=  *=  /=  %=  !=      &=  |=  <=  >=  .=  :=
		->                  =>
```

## Delimiters

```
() {} []
```

## Booleans

```
true false
```

## Numbers

**int**

```
0 10 1000 1000000 10000000000
```

**float**

```
0.5 1.0 230.47 30949.374
```

## Chars

```
'a' 'b' 'c'
'b' 'c' 'd'
'c' 'd' 'e'
```

## Strings

```
"hello, world! 👽"
"你好!"
"hello, \"world\"! 👽\nwesh la famille! 🤘"
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
action  as        async       await   bench   bind
break   chan      continue    else    enum    exp
ext     false     fun         for     if      imu
load    loop      match       Me      me      mock
mod     mut       pub         ref     return  set
spawn   struct    test        type    unit    val
void    wasm      where       while
```

## Types

**primitive types**

```
u8    u16   u32   u64   uint   
s8    s16   s32   s64   sint    
f32   f64   str   char  bool    
```

**derived types**

```
type Symbol = s32;
```

## Arrays

```
[10, 23, 12, 3]
```

## Bindings

`imu` immutable constant variable keyword   
`val` immutable local variable keyword    
`mut` mutable local variable keyword    

**preview**

```
imu x : uint = 1;
val y : uint = 2;
mut z : uint = 3;
```

**inferred types**

```
val a := true;
val b := false;
val c := a == b;
```

**multiple assignments**

```
val x, y, z, : s8 = 0;
val x, y, z, := 0;
```

## Functions

`fun` function declaration    
`\->` arrow function    

```
fun square : (uint) -> uint = (x) { x * x }
val square : \ uint -> uint = \ x -> x * x;
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
for 0..4 = (num) {}
for nums = (num) {}
```

**while**

```
while true {}
while 0 < 1 {}
```

## Structures

```
struct Button {
  id: uint,
  name: str,
}

set Button {
  fun new: (str) -> Button = (name) {
    Button {
      id = 0,
      name = name,
    }
  }
}
```

## Actions

actions are simply interfaces that allow a new behaviour to be assigned 

```
action Vec2 {
  fun mul: (uint) -> uint;
}

struct Point {
  x: f32,
  x: f32,
}

set Vec2 for Point {
  fun mul: (uint) -> uint = (x) { x * x }
}
```

## Modules

`bind` - set module keyword   
`load` - get module keyword   

**bind**

```
pub bind matrix;
bind opengl;
```

**load**

```
load std::regex;
load lib::matrix::perspective;
load lib::matrix::math::(min, max);
```

## Matching

```
match op {
  '+' => add(x, y),
  '-' => sub(x, y),
  '*' => mul(x, y),
  '/' => div(x, y),
  _ => #panic("with msg"),
}
```

Attributes

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

## Macros

```
macro foo := () {}

#foo();
```

## Asynchronous

`async` - make asynchronous keyword
`await` - make thread keyword

```
async {
  await 0
}
```

## Channels

`chan` - make channel keyword   
`spawn` - make thread keyword   

*still in research*

```
chan (tx, rx) = 0;
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
  -- add mocks and tests
}
```

**mock**

```
mock tokenization_mock = () {
  -- mock computation
}
```

**test**

```
test tokenization_test = () {
  -- test computation
}
```

## Benchmarks

`bench` - bench computation keyword   

**bench**

```
bench tokenization_benchmark = () {
  -- bench computation
}
```

## FFI

`ext` - call a c function keyword   
`exp` - call a javascript function keyword    
`mod` - call a rust function keyword    

**c**

```
ext fun cos: (uint) -> uint;
```

**javascript**

```
exp fun cos: (uint) -> uint;
```

**rust**

```
mod fun cos: (uint) -> uint;
```

## WebAssembly

`wasm` - represents webassembly keyword   

**wasm**

```
wasm fun bar := () {
  -- something to do
}
```
