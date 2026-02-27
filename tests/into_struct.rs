#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};

use inew::New;

#[test]
fn struct_into() {
    #[derive(New)]
    struct A {
        #[new(into)]
        x: String,
    }

    let res = A::new("abc");
    assert_eq!(res.x, "abc".to_string());
}

#[test]
fn tuple_struct_into() {
    #[derive(New)]
    struct A(#[new(into)] String);

    let res = A::new("abc");
    assert_eq!(res.0, "abc".to_string());
}
