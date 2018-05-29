// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Library for wrapping a block or expression into a closure and call it
//!
//! ## Introduction
//!
//! The functionality of this crate was defined, because every now and then it is unpractical,
//! that the `?` operator in rust has a "unwrap or return" semantic. In many other languges
//! (such as Swift, C#, Kotlin, etc.) the `?.` operator allows "safe navigation",
//! and this would be practical for the `Option` and `Result` type in Rust every now and then.
//! The navigation code could be extracted to another function,
//! but oftentimes this would fragment code that actually belongs together.
//! Another workaround is to wrap a `?.` call chain into a closure and
//! call it directly. For example, consider the following piece of code:
//!
//! ```rust
//! let o = Some("Foobar");
//! let s = o.and_then(|st| st.get(0..3)).map(|st| st.to_lowercase());
//! assert_eq!("foo", s.unwrap());
//! ```
//!
//! The second line would be more readable using the `?.` navigation. But this would lead
//! to a return from the entire function if an empty option is being accessed. So the call
//! chain can be wrapped into a cosure which is then immediately called. The following code
//! shows, how the previous example can be written in this style:
//!
//! ```rust
//! let o = Some("Foobar");
//! let s = (|| Some(o?.get(0..3)?.to_lowercase()) )();
//! assert_eq!("foo", s.unwrap());
//! ```
//!
//! While this code is very terse, it is not necessarily the easiest to read.
//! The `fn_block` crate provides functionality to make the code above
//! a little easier to read:
//!
//! ```rust
//! # #[macro_use]
//! # use fn_block::*;
//! let o = Some("Foobar");
//! let s = fn_expr!{ o?.get(0..3)?.to_lowercase().into_some() };
//! assert_eq!("foo", s.unwrap());
//! ```
//!
//! It was considered to provide a version of the macros that automatically
//! calls `into()` on the result of the expression/block. This would allow an automatic
//! conversion of a value to the actual return type, provided a fitting implementation
//! of the `Into` trait was in scope. But this was considered too implicit. The `?` operator
//! already performs implicit conversion of error types; this seems to be enough magic
//! to grok for the user.


///////////////////////
// Macro definitions //
///////////////////////

/// This macro wraps a given rust code block into a closure and
/// directly calls the closure. Optionally the return type of the
/// closure can be specified first and separeted with a colon from
/// the body block.
///
/// # Example without return type:
///
/// ```rust
/// # #[macro_use]
/// # use fn_block::*;
/// let o = Some("Foobar");
/// let s = fn_expr!{ o?.get(0..3)?.to_lowercase().into_some() };
/// assert_eq!("foo", s.unwrap());
/// ```
///
/// # Example with return type:
///
/// ```rust
/// # #[macro_use]
/// # use fn_block::*;
/// use std::str::from_utf8;
/// use std::error::Error;
/// struct ConvertErr();
/// impl <T: Error> From<T> for ConvertErr {
/// 	fn from(_: T) -> ConvertErr {ConvertErr()}
/// }
/// let five : &[u8] = &[0x0020,0x0034,0x0032];
/// let res_int = fn_block!{Result<u32,ConvertErr>: {
/// 	let str = from_utf8(five)?.trim();
/// 	str.parse::<u32>()?.into_ok()
/// }}.unwrap_or(0u32);
/// assert_eq!(res_int, 42);
/// ```
///
/// Note that the examples use the traits [`IntoSome`] and [`IntoOk`],
/// Defined in this crate.
///
/// [`IntoSome`]: trait.IntoSome.html
/// [`IntoOk`]: trait.IntoOk.html
#[macro_export]
macro_rules! fn_block {
	($return_type:ty : $body:block) => {
		(|| -> $return_type { $body })()
	};
	($body:block) => {
		(|| $body)()
	};
}

/// This macro wraps a given rust code block into a closure and
/// directly calls the closure. Optionally the return type of the
/// closure can be specified first and separeted with a colon from
/// the body expression.
///
/// # Example without return type:
///
/// ```rust
/// # #[macro_use]
/// # use fn_block::*;
/// let o = Some("Foobar");
/// let s = fn_block!{{
///   let foo = o?.get(0..3);
///   Some(foo?.to_lowercase())
/// }};
/// assert_eq!("foo", s.unwrap());
/// ```
///
/// # Example with return type:
///
/// ```rust
/// # #[macro_use]
/// # use fn_block::*;
/// use std::str::from_utf8;
/// use std::error::Error;
/// struct ConvertErr();
/// impl <T: Error> From<T> for ConvertErr {
/// 	fn from(_: T) -> ConvertErr {ConvertErr()}
/// }
/// let s : &[u8] = &[0x0020,0x0034,0x0032];
/// let res_int = fn_expr!{ Result<u32,ConvertErr>:
/// 	from_utf8(s)?.trim().parse::<u32>()?.into_ok()
/// }.unwrap_or(0u32);
/// assert_eq!(res_int, 42);
/// ```
///
/// Note that the example use the trait [`IntoOk`],
/// defined in this crate.
///
/// [`IntoOk`]: trait.IntoOk.html
#[macro_export]
macro_rules! fn_expr {
	($return_type:ty : $body:expr) => {
		(|| -> $return_type { $body })()
	};
	($body:expr) => {
		(|| $body)()
	};
}

///////////////////////
// Trait definitions //
///////////////////////

/// This trait, which is implemented for all sized types,
/// provides the method `into_some`, which moves the
/// value on which it is called into an `Optional::Some`.
/// This is particularly useful when having to wrap a value into
/// a `Some` at the end of a call chain.
///
/// # Example:
///
/// ```rust
/// # #[macro_use]
/// # use fn_block::*;
/// let o : Option<String> = "foo bar ".trim().to_uppercase().into_some();
/// assert_eq!("FOO BAR", o.unwrap());
/// ```
///
/// This can e.g. be used inside of
/// an expression wrapped in a [`fn_expr!`] or [`fn_block!`] macro.
///
/// # Example using `fn_expr!`:
///
/// ```rust
/// # #[macro_use]
/// # use fn_block::*;
/// let o = Some("Foobar");
/// let s = fn_expr!{ o?.get(0..3)?.to_lowercase().into_some() };
/// assert_eq!("foo", s.unwrap());
/// ```
///
/// [`fn_expr!`]: macro.fn_expr.html
/// [`fn_block!`]: macro.fn_block.html
pub trait IntoSome: Sized {
	fn into_some(self) -> Option<Self>;
}

/// Implementration of trait `IntoSome` for
/// all sized types.
///
/// # Example
///
/// ```rust
/// # use fn_block::IntoSome;
/// let five = 5.into_some();
/// assert_eq!(Some(5), five);
/// ```
///
impl<T> IntoSome for T {
	fn into_some(self) -> Option<Self> {
		Some(self)
	}
}

/// This trait, which is implemented for all sized types,
/// provides the method `into_ok`, which moves the
/// value on which it is called into an `Result::Ok`.
/// This is particularly useful when having to wrap a value into
/// an `Ok` at the end of a call chain.
///
/// # Example:
///
/// ```rust
/// # #[macro_use]
/// # use fn_block::*;
/// let res : Result<String,()> = "foo bar ".trim().to_uppercase().into_ok();
/// assert_eq!("FOO BAR", res.unwrap());
/// ```
///
/// This can e.g. be used inside of
/// an expression wrapped in a [`fn_expr!`] or [`fn_block!`] macro.
///
/// # Example using `fn_expr!`:
///
/// ```rust
/// # #[macro_use]
/// # use fn_block::*;
/// use std::str::from_utf8;
/// use std::error::Error;
/// struct ConvertErr();
/// impl <T: Error> From<T> for ConvertErr {
/// 	fn from(_: T) -> ConvertErr {ConvertErr()}
/// }
/// let s : &[u8] = &[0x0020,0x0034,0x0032];
/// let res_int = fn_expr!{ Result<u32,ConvertErr>:
/// 	from_utf8(s)?.trim().parse::<u32>()?.into_ok()
/// }.unwrap_or(0u32);
/// assert_eq!(res_int, 42);
/// ```
///
/// [`fn_expr!`]: macro.fn_expr.html
/// [`fn_block!`]: macro.fn_block.html
pub trait IntoOk<E>: Sized {
	fn into_ok(self) -> Result<Self, E>;
}

/// Implementration of trait `IntoOk` for
/// all sized types.
///
/// # Example
///
/// ```rust
/// # use fn_block::IntoOk;
/// let five : Result<u32,()> = 5.into_ok();
/// assert_eq!(Ok(5), five);
/// ```
///
impl<T, E> IntoOk<E> for T {
	fn into_ok(self) -> Result<Self, E> {
		Ok(self)
	}
}

#[macro_use]
#[cfg(test)]
mod tests;
