# Design

## Type System

### AXIOMS for types:

  * A type can be `primitive`, `structure`, `function` or `reference`.
  * The program cannot define new `primitive` types
  * The `primitive` types provided are :

  ```
  uint, u8, u16, u32, u64, sint, s8, s16, s32, s64, bool, f32, f64, char, str
  ```

  * The program can define a `structure`
  * A structure is a list of fields associated with a type (itself a `primitive`, `structure`, `function` or `reference`)
  * The definition of a field in an `A` structure cannot contain (directly, or via its own fields and their subfields) another structure of type `A`.
  * A reference must be associated with another `primitive` type, `structure`, `function` or `reference`.
  * The program can define a `function`

### Type categories:
  * `P` : primitive
  * `S(c1: X1, c1: X2, ..., cn: Xn)`: structure where `X1`, ..., `Xn` are types, `c1`, ..., `cn` are identifiers and `n >= 1` is any.
  * `R(X)` : reference, where `X` is another type
  * `F(X0, X1, ..., Xn)` : function where `X0`, ..., `Xn` are types (`X0` return type) and `n >= 0` is any

### Construction rules:
  * `P` is in { `uint`, `u8`, `u16`, `u32`, `u64`, `sint`, `s8`, `s16`, `s32`, `s64`, `bool`, `f32`, `f64`, `char`, `str` }
  * direct dependencies:
    * for an X of category `S(c1 : X1, ..., cn : Xn)`, `D(X) = { X1, X2, ..., Xn } union D(X1) union ... union D(Xn)`
    * for an `X` of category `P` or `R(Y)`,
      `D(X) = { }`
  * For any type `X`, `X` must not belong to `D(X)`

### Substitution rules in an expression:
  * `R(X) -> X`
  * `S(c1 : X1, ..., cn : Xn).ck -> Xk (where 1 <= k <= n)`
