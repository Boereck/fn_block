# `fn_block` Version History

## 0.1.0 (2018-05-25)

* Added `fn_expr` macro
* Added `fn_block` macro
* Added `IntoOk` trait and implementation for every type
* Added `IntoSome` trait and implementation for every type

## 0.2.0 (2018-07-04)

* Added unstable/unproven marco `fn_try` (crate feature `unproven` needs to be enabled)
* Deprecated `fn_block` macro, since `fn_expr` can also wrap blocks. This is a bit ironic, since the crate name is `fn_block`

## 0.2.1 (2018-07-04)

* Only crate `Cargo.toml` metadata fixes