# `fn_block` Library

Library defining macros for calling blocks or expressions in a closure.

## Quick Introduction

This library was mostly written to allow "safe navigation" with the `?.` operator combination
(seemingly) without jumping out of the current function. This allows a similar use of the operator
as in other languages (such as Swift, C# or Kotlin).

To use this library, you have to add it to the dependencies of your `Cargo.toml` file

```toml
[dependencies]
fn_block = "0.1.0"
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

As soon as the API documentation is published, a link to the crate documentation
will be added here.

# Functionality Overview

In short, this crate provides the following APIs:

* The `fn_expr` macro allows wrapping an expression into a lambda that is directly called.
* The `fn_block` macro allows wrapping a block into a lambda that is directly called.
* The `IntoSome` trait, which is implemented for all `Sized` types, allows to call `into_some` 
  on a value to move it into an `Option::Some`.
* The `IntoOk` trait, which is implemented for all `Sized` types, allows to call `into_ok` 
  on a value to move it into an `Result::Ok`.

As soon as the API documentation is published it will be updated here.  
For more examples, please have a look at the test module.