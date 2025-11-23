# calc-generic

A Rust experiment exploring generics and trait bounds/constraints through a calculator REPL.

## Overview

This project demonstrates:
- Generic structs with multiple trait bounds
- Associated functions on generic types
- Static type registry using `LazyLock<HashMap>`
- Memory layout analysis with `memoffset` and `static_assertions`
- Heap profiling with `dhat`

## Usage

```bash
cargo run
```

Enter calculations using type suffixes:

```
> 24_i8 123     # i8 arithmetic
> 100_u32 50    # u32 arithmetic
> 10 20         # defaults to u8
> .quit         # exit
```

## Example Output

```
> 24_i8 123
24 + 123 = None
24 - 123 = Some(-99)
24 * 123 = None
24 / 123 = Some(0)

  0b 0001 1000 (0x18) (24)
& 0b 0111 1011 (0x7B) (123)
= 0b 0001 1000 (0x18) (24)

  0b 0001 1000 (0x18) (24)
| 0b 0111 1011 (0x7B) (123)
= 0b 0111 1011 (0x7B) (123)

  0b 0001 1000 (0x18) (24)
^ 0b 0111 1011 (0x7B) (123)
= 0b 0110 0011 (0x63) (99)
```

## Supported Types

`i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`

## Project Structure

```
src/
  lib.rs        # Module declarations and re-exports
  main.rs       # REPL loop and profiling setup
  traits.rs     # IntegerOps trait definition
  calculator.rs # Calculator struct and Display impl
  format.rs     # Binary formatting utilities
  types.rs      # Command enum and TYPES registry
```

## Type Layouts

The program prints memory layouts at startup:

```
=== Type Layouts ===

Calculator<i8>:
  size:  2 bytes <-------------- Total bytes
  align: 1 bytes
  op1:   offset 0, size 1   <--: Sum to 
  op2:   offset 1, size 1   <--: 2 bytes

Calculator<i16>:
  size:  4 bytes
  align: 2 bytes
  op1:   offset 0, size 2
  op2:   offset 2, size 2

Calculator<i32>:
  size:  8 bytes
  align: 4 bytes
  op1:   offset 0, size 4
  op2:   offset 4, size 4

Calculator<i64>:
  size:  16 bytes
  align: 8 bytes
  op1:   offset 0, size 8
  op2:   offset 8, size 8

Calculator<i128>:
  size:  32 bytes
  align: 16 bytes
  op1:   offset 0, size 16
  op2:   offset 16, size 16

TYPES (HashMap<&str, ParseFn>):
  size:  48 bytes
  align: 8 bytes

LazyLock<HashMap<...>>:
  size:  56 bytes
  align: 8 bytes

Entry (&'static str, ParseFn):
  &str size:   16 bytes
  ParseFn size: 8 bytes
  total per entry: 24 bytes

===================
```

## Profiling

Heap allocation stats are printed on exit via `dhat`. View detailed analysis:

```bash
# Opens dhat-heap.json in browser
open https://nnethercote.github.io/dh_view/dh_view.html
```
