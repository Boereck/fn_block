use super::*;

/// Trait implemented for all sized types, providing a `when` function,
/// wrapping the element it is called on into an `Optional::Some` if the
/// given predicate holds true for the value and `Optional::None` otherwise.
trait Optionalize: Sized {
    fn when<F>(self, predicate: F) -> Option<Self>
    where
        F: FnOnce(&Self) -> bool,
    {
        if predicate(&self) {
            Some(self)
        } else {
            None
        }
    }
}

/// Implementation of Optionalize for all values
impl<T> Optionalize for T {}

#[test]
fn fn_block_some() {
    let o: Option<i32> = Some(42);
    let foo = fn_block!{{
        let gt_zero = o?.when(|&i| i > 0);
        gt_zero?.when(|&i| i%2 == 0)
    }};
    assert_eq!(42, foo.expect("result"));
}

#[test]
fn fn_block_none() {
    let o: Option<i32> = Some(42);
    let foo = fn_block!{{
        let big_num = o?.when(|&i| i > 1000);
        big_num?.when(|&i| i < 2000)
    }};
    assert!(foo.is_none());
}

#[test]
fn fn_expr_block_some() {
    let o: Option<i32> = Some(42);
    let foo = fn_expr!{{
        let gt_zero = o?.when(|&i| i > 0);
        gt_zero?.when(|&i| i%2 == 0)
    }};
    assert_eq!(42, foo.expect("result"));
}

#[test]
fn fn_expr_block_none() {
    let o: Option<i32> = Some(42);
    let foo = fn_expr!{{
        let big_num = o?.when(|&i| i > 1000);
        big_num?.when(|&i| i < 2000)
    }};
    assert!(foo.is_none());
}

#[test]
fn fn_expr_some() {
    let o: Option<i32> = Some(42);
    let foo = fn_expr!{ o?.when(|&i| i > 0)?.when(|&i| i%2 == 0) };
    assert_eq!(42, foo.expect("result"));
}

#[test]
fn fn_expr_none() {
    let o: Option<i32> = Some(42);
    let foo = fn_expr!{ o?.when(|&i| i > 1000)?.when(|&i| i < 2000) };
    assert!(foo.is_none());
}

#[test]
fn fn_block_resulttype() {
    use std::num::ParseIntError;
    let res = fn_block!{ Result<u32,ParseIntError>: {
        let s = "4711";
        s.parse()
    }};
    assert_eq!(4711, res.unwrap());
}

#[test]
fn fn_expr_resulttype() {
    use std::num::ParseIntError;
    let res = fn_expr!{ Result<u32,ParseIntError>: "4711".parse() };
    assert_eq!(4711, res.unwrap());
}

#[test]
fn into_ok() {
    let r: Result<&str, u16> = "foo".into_ok();
    assert_eq!("foo", r.expect("result is Ok"));
}

#[test]
fn into_some() {
    let r: Option<u32> = 42.into_some();
    assert_eq!(42, r.expect("result is Some"));
}

#[test]
fn showcase() {
    // Not an actual test, but a showcase for several alternative ways to express the same
    let o = Some("Foobar");
    let s = o.and_then(|st| st.get(0..3)).map(|st| st.to_lowercase());
    assert_eq!("foo", s.expect("result is Some"));

    let s = (|| Some(o?.get(0..3)?.to_lowercase()))();
    assert_eq!("foo", s.expect("result is Some"));

    use super::IntoSome;
    let s = fn_expr!{ o?.get(0..3)?.to_lowercase().into_some() };
    assert_eq!("foo", s.expect("result is Some"));
}

use std::num::ParseIntError;
use std::str::Utf8Error;

enum ConvertErr {
    StrParseErr,
    IntParseErr,
}

impl From<Utf8Error> for ConvertErr {
    fn from(_: Utf8Error) -> ConvertErr {
        ConvertErr::StrParseErr
    }
}
impl From<ParseIntError> for ConvertErr {
    fn from(_: ParseIntError) -> ConvertErr {
        ConvertErr::IntParseErr
    }
}

#[test]
#[cfg(feature = "unproven")]
fn fn_catch_result() {
    use std::str::from_utf8;

    let s: &[u8] = &[0x0020, 0x0034, 0x0032];
    let i = fn_try! {
        from_utf8(s)?.trim().parse::<u32>()?
        => catch {
            ConvertErr::StrParseErr => 0u32,
            ConvertErr::IntParseErr => u32::max_value()
        }
    };
    assert_eq!(42, i);
}

#[test]
#[cfg(feature = "unproven")]
fn fn_catch_error() {
    use std::str::from_utf8;

    let s: &[u8] = &[0x0020, 0x005A, 0x0032];
    let i = fn_try! {
        from_utf8(s)?.trim().parse::<u32>()?
        => catch {
            ConvertErr::StrParseErr => 0u32,
            ConvertErr::IntParseErr => u32::max_value()
        }
    };
    assert_eq!(u32::max_value(), i);
}
