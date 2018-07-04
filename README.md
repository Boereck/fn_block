[![Build Status](https://travis-ci.org/Boereck/fn_block.svg?branch=master)](https://travis-ci.org/Boereck/fn_block) 
[![AppVeyor Status](https://ci.appveyor.com/api/projects/status/apctgp7w8qcwttag?svg=true)](https://ci.appveyor.com/project/Boereck/fn-block) 
[![Crates.io Version](https://img.shields.io/crates/v/fn_block.svg)](https://crates.io/crates/fn_block)

# `fn_block` Crate

Library defining macros for calling blocks or expressions in a closure.

## Quick Introduction

This library was mostly written to allow "safe navigation" with the `?.` operator combination
(seemingly) without jumping out of the current function. This allows a similar use of the operator
as in other languages (such as Swift, C# or Kotlin).

To use this library, you have to add it to the dependencies of your `Cargo.toml` file

```toml
[dependencies]
fn_block = "0.2.0"
```

Then add the following lines to your module:

```rust
#[macro_use]
extern crate fn_block;
use fn_block::*;
```
Instead of the wildcard, you can also just import the symbols you need. 

Here is an example on how to use the crate:

```rust
let o = Some("Foobar");
let s = fn_expr!{ o?.get(0..3)?.to_lowercase().into_some() };
assert_eq!("foo", s.unwrap());
```

Please visit the [API Documentation](https://docs.rs/fn_block/latest/) for more details.

## Functionality Overview

In short, this crate provides the following APIs:

* The [`fn_expr`] macro allows wrapping an expression into a lambda that is directly called.
* The [`IntoSome`] trait, which is implemented for all `Sized` types, allows to call [`into_some`] 
  on a value to move it into an `Option::Some`.
* The [`IntoOk`] trait, which is implemented for all `Sized` types, allows to call [`into_ok`] 
  on a value to move it into an `Result::Ok`.

For more examples, please have a look at the test module.

## Unstable Features

To use unstable features, the dependency declaration in your `Cargo.toml` has to be updated:

```toml
[dependencies]
fn_block = { version = "0.2.0", features = ["unproven"] }
```
Note that this crate's unstable features *do* work on stable Rust.

The following unstable APIs are available:

* The [`fn_try`] macro allows wrapping an expression into a lambda, being called directly and recover from errors directly afterwards.

## License

The fn_block crate is licensed under the following licenses:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) / http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) / http://opensource.org/licenses/MIT)

Choose under which you want to use the library.

[`fn_expr`]: https://docs.rs/fn_block/latest/fn_block/macro.fn_expr.html
[`fn_block`]: https://docs.rs/fn_block/latest/fn_block/macro.fn_block.html
[`fn_try`]: https://docs.rs/fn_block/latest/fn_block/macro.fn_try.html
[`IntoSome`]: https://docs.rs/fn_block/latest/fn_block/trait.IntoSome.html
[`into_some`]: https://docs.rs/fn_block/latest/fn_block/trait.IntoSome.html#tymethod.into_some
[`IntoOk`]: https://docs.rs/fn_block/latest/fn_block/trait.IntoOk.html
[`into_ok`]: https://docs.rs/fn_block/latest/fn_block/trait.IntoOk.html#tymethod.into_ok