# qhantoom compiler: syntax

defining a relevant syntax that makes semantic sense is not easy. many programming language designers neglect this aspect completely. I have often read and heard that syntax: "who cares". I have a strong focus on design, that's one of the reasons why I don't agree with this kind of statements. my naive vision is probably misleading me. But I think that a syntax must be simple, coherent, intuitive and above all flexible *(a bit like LEGOS).*   

Semantics is the most important thing in a language *(semantic is king)* but design also plays an important role because it's the design that carries the semantics. one can't go without the other, for example it's like making a song by focusing on the content without thinking about the form. It's by doing this that you miss out on a hit or a banger. It is this balance between the two that makes a programming language taste good.    

The syntax of `qhantoom` is intended to be similar to the highway code. It should allow the programmer to write clear instructions. Allowing better distinction and implementation of certain functional paradigms for the uninitiated.   

The idea is that a symbol has only one use depending on its context. This avoids things like: ``` Math.cos(2) === Math.cos`2` ``` returns `true` in `javascript`. Having two different ways of declaring the call of a function is confusing. And that's what I want to avoid with my compiler.    

My syntax choices are focused on the areas of improvement that I have found in some programming languages. These choices are mine alone and are not the way to go.    

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
:=
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

`imu` - immutable global variable keyword    
`val` - immutable local variable keyword   
`mut` - mutable local variable keyword   

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
val x y z: s8 = 0;
val x y z := 0;
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
for 0..4 = (num) {}
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
fun sqrt: (u8) -> u8 = (x) { x * x }

# closure expression
val sqrt: (u8) -> u8 = (x) { x * x }

# closure expression
mut sqrt: (u8) -> u8 = (x) { x * x }

# arrow function expression
mut sqrt: u8 -> u8 = \ x -> x * x;
```

## Arrays

```
[10, 23, 12, 3]
```

## Structures

`struct`

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

val button: Button = Button {
  id = 0,
  name = "button-name",
};

val button: Button = new::Button("button-name");
```

## Recursions

**factorial**

```
fun factorial: (uint) -> uint = (i) {
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
  fun mul: (uint) -> (uint);
}

struct Point {
  x: f32,
  x: f32,
}

set Vec2 for Point {
  fun mul: (uint) -> (uint) = () {}
}
```

## Modules

`bind` - set module keyword   
`load` - get module keyword   

**bind | pack**

```
pub bind ident;
pub bind ident;
bind ident;
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

`ext` - call a **c** function keyword   
`exp` - call a **javascript** function keyword    
`mod` - call a **rust** function keyword    

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
wasm fun main := () {
  # something to do
}
```
