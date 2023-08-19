# Learning Rust by Doing It All Wrong.

Your mission, if you choose to accept it, is to refactor this {awful?, awesome?} port of clox from Robert Nystrom's [Crafting Interpreters](http://www.craftinginterpreters.com/) into safe, idiomatic Rust.

As a straight, line-by-line port from C to Rust (rust-c?, c-rust?, crusty?, pick your poison) with over 1500 `unsafe` blocks[^*] (90% of functions, >50% of SLOC), `mut` everywhere, pointer arithmetic, memory aliasing, global mutable state, type punning, intrusive lists, unions, uninitialized memory! there is plenty to cringe about.
Jump into the deep end of Rust and learn to swim.

A big thanks to Robert Nystrom for open sourcing his excellent book [Crafting Interpreters](https://github.com/munificent/craftinginterpreters), without which this crazy project would not been possible.

- Developed with rustc 1.70.0-x86_64-apple-darwin (90c541806 2023-05-31).
- Test files from and application code based on [https://github.com/munificent/craftinginterpreters]() (01e6f5b8f 2022-02-17).

[^*]: ~350 `unsafe` blocks are nested within another `unsafe` block or function and therefore not required but `#![allow(unused_unsafe)]` and `#![deny(unsafe_op_in_unsafe_fn)]` are used to permit them and call them out since each one requires the caller to meet the safety requirements of the unsafe operations used. An additional ~150 `unsafe` blocks are left out of these counts and are consolidated with other `unsafe` blocks because they add a significant amount of noise but add little value.

## Other things to try:

- Diff against the book C version (ignoring whitespace). Which parts surprise you with how similar they are?
- What code are you surprised not to be wrapped in `unsafe`?
- `parse_rules!` statically allocates an uninitialized array and then initializes it by index. Would you even expect this to be possible given the limitations of static initializers? What happens if you reorder the rules? Define the same index twice? Leave some out? Add some out of bounds?
- Which raw pointers can be replaced with reference pointers with only minimal refactoring?
- Where can slices be utilized instead of raw pointers with only minimal refactoring?
- Which uses of `reallocate` can be replaced with `Box` pointers with only minimal refactoring?
- Is the number of calls to `clone` higher or lower than you expect? Other than reduce the number of explicit calls to `clone`, what impact would `impl Copy` have?
- What parts of the code are idiomatic Rust, if any?
- See what Clippy says about it. What stands out to you?
- See what Miri says about it. What stands out to you?
- See what Valgrind says about it. What stands out to you?
- Which `unsafe` blocks/functions are not actually unsafe, if any?
- Which `unsafe` blocks are still unsafe, if any?
- Which `unsafe` functions can be made safe by adding parameter validation and panicking as the defined behavior?
- Add `SAFETY` comments to `unsafe` functions documenting the requirements its callers must meet in order to ensure defined behavior.
- Add `SAFETY` comments to `unsafe` blocks documenting how it meets the safety requirements of each `unsafe` operation it uses.
- What can be done to safely reduce the number of places where `unsafe` is required (not just the keyword) without changing any types in data structures or function signatures?
- Which uses of `unsafe` can be replaced with only minimal refactoring and without sacrificing performance?
- Annotate each raw pointer with a comment on its lifetime then replace each with a reference to a newtype which wraps it. Compare your lifetime comments to what you were able to get to compile?
- What issues can arise from using `unsafe` inside macros?
- Compare the generated machine code against the book C version. Which parts surprise you with how similar or different they are? Can you find ways to increase the performance of either?
- What calls to a checked function can be switched to an unsafe unchecked function to increase performance? How much impact does that have?
- Remove `#![allow(unused_mut)]` and resolve issues. How much impact does that have on performance? Which parts of the code benefit the most from optimizations related to immutability?
- How else could the Obj* types be implemented in Rust?
- What problems can arise from the indiscriminate use of wildcard imports and reexports? How else could `#include` and header files been simulated?
- Show adjustments with Rust Analyzer (`rust-analyzer.inlayHints.expressionAdjustmentHints.enable`). What stands out to you? Are you surprised how often rust coerces a value to a different type?
- Which explicit type coercions can be elided?
- `std` does not expose `strlen` or `memcmp` nor does this project link with a library which does, so where do they come from?
- Write a program that will extract the code for each chapter with and without the code tags marking each snippet. Add support for extracting the code at the point each snippet was added (you'll need the book source for the ordering of snippets).

## Tips
### run vm
```shell
    cargo run [lox-file]
```
### run test suite from book
```shell
    cargo test
```
### strip code tags
```shell
    sed -i.orig -e '/^\/\* /,/^\*\//d' -e '/^\/\/[<>] /d' src/*.rs
```

----
All material in this repository is licensed under the [MIT License](https://spdx.org/licenses/MIT.html).
