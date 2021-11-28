# Syntax

## Comments

`--` line comment    

`-%` - comment block scope start   
`%-` - comment block scope end   

`---` doc line comment    

`--!` top-level doc line comment   

`--!` - block doc comment start   
`!--` - block doc comment end   

```
-- this is a line comments
-% this is a block comments %-
--- this is a doc line comments
--! this is a top-level doc comments
--! this is a doc comments !--
```

## Shebang comments

```
#!/usr/bin/env qhantoomc
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
break   chan      continue    else    enum    ext     
false   fun       for         if      imu     load    
loop    match     Me          me      mock    mod     
mut     pub       ref         req     return  set
spawn   struct    test        type    unit    val
void    wasm      while
```

## Types

**primitive types**

```
u8    u16   u32   u64   uint   
s8    s16   s32   s64   sint    
f32   f64   str   char  bool    
```

## Operators

```
+   -   *   /   %   !   =   &   |   <   >   .   :   ;   #   ?   @   _
                        ==  &&  ||  <<  >>  ..  ::
+=  -=  *=  /=  %=  !=      &=  |=  <=  >=  .=  :=
		->                  =>          <-
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

**decimal**

```
0 10 1000 1000000 10000000000
```

**binary**

```
0b0111
```

**octal**

```
0o72
```

**hexadecimal**

```
0xfff
```

**float**

```
0.5 1.0 230.47 30949.374
```

**separators**

```
1_000_000_000_000
0b1010_0001_1000_0101
0o7_7_3_4
0xf_f_f_f_f
```

## Arrays

```
[10, 23, 12, 3]
```

## Tuples

```
(1, 2, 3)
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

**derived types**

```
type Symbol = s32;
```

## Bindings

`val` constant variable keyword (constant are known at compile-time)   
`imu` immutable local variable keyword    
`mut` mutable local variable keyword    

**preview**

```
val y : uint = 1;
imu x : uint = 2;
mut z : uint = 3;
```

**inferred types**

```
imu a := true;
imu b := false;
imu c := a == b;
```

**multiple assignments**

```
imu x, y, z : s8 = 0;
imu x, y, z := 0;
```

## Functions

`fun` function declaration    
`fn` arrow function declaration    

```
fun square(x : uint) : uint { x * x }
imu square : Fn(x : uint) -> uint = fn (x : uint) -> x * x;
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
  #abort()
}
```

**ternary**

```
when true ? 1 : 0;
```

## Loops

**loop**

```
loop {}
```

**for**

```
-- longhand
for n := 0..3 {
  n += 1
}
```

```
-- shorthand
for n := 0..3 -> n += 1;
```

**until**

the exiting condition is executed after each loop   

```
until true {}
until 0 < 1 {}
```

**while**

the exiting condition is executed before each loop    

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

impl Button {
  fun new(name : str) : Button {
    Button {
      id = 0,
      name = name,
    }
  }
}
```

## Traits

**wip**

`behaviors` are simply interfaces that allow a new behavior to be assigned 

```
behavior Vec2 {
  fun mul(x : uint) : uint;
}

struct Point {
  x: f32,
  x: f32,
}

impl Vec2 for Point {
  fun mul(x : uint) : uint { x * x }
}
```

## Modules

`bind` - set module keyword   
`load` - get module keyword   

**bind**

```
bind opengl;
pub bind matrix;
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
  _ => #abort("with msg"),
}
```

## Attributes

```
#> repr: c.
#> derive: clone, debug.
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
macro foo() {
  () {}
}

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

```
-- send a value from a channel
fun thread(c : chan uint, value : uint) {
  c <- value;
}

-- declare a channel
imu c : uint = chan 0;

spawn thread(c, 3);

-- retrieve a value from a channel
imu x : uint = <-x;

-- prints 3
#print("received: {}", x);
```

## Assertions

`unit` - assertion unit keyword   
`mock` - assertion mock keyword   
`test` - assertion test keyword   
`must` - assertion must keyword   

**unit**

```
unit {
  -- add mocks and tests
}
```

**mock**

```
mock tokenization_mock() {
  -- mock computation
}
```

**test**

```
test tokenization_test() {
  -- test computation
}
```

**must**

```
unit {
  test my_test() {
    #must(0 eq 0);
  }
}
```

## Benchmarks

`bench` - bench computation keyword   

**bench**

```
bench tokenization_benchmark() {
  -- bench computation
}
```

## FFI

`ext` - call a c function keyword   
`req` - call a javascript function keyword    
`mod` - call a rust function keyword    

**c**

```
ext fun cos(x : uint) : uint;
```

**javascript**

```
req fun cos(x : uint) : uint;
```

**rust**

```
mod fun cos(x : uint) : uint;
```

## WebAssembly

`wasm` - represents webassembly keyword   

**wasm**

```
wasm fun bar() {
  -- something to do
}
```
