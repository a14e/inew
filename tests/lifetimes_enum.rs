#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};

use inew::New;

#[test]
fn enum_with_lifetimes() {
    #[derive(Debug, PartialEq, New)]
    enum A<'a> {
        I { x: &'a u64 },
    }

    let x = 1u64;
    let res = A::new_i(&x);
    assert_eq!(res, A::I { x: &x });
}

#[test]
fn const_enum_with_lifetimes() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A<'a> {
        I { x: &'a u64 },
    }

    const X: u64 = 1u64;
    const RES: A = A::new_i(&X);
    assert_eq!(RES, A::I { x: &X });
}

#[test]
fn tuple_enum_with_lifetimes() {
    #[derive(Debug, PartialEq, New)]
    enum A<'a> {
        I(&'a u64),
    }

    let x = 1u64;
    let res = A::new_i(&x);
    assert_eq!(res, A::I(&x));
}

#[test]
fn const_tuple_enum_with_lifetimes() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A<'a> {
        I(&'a u64),
    }

    const X: u64 = 1u64;
    const RES: A = A::new_i(&X);
    assert_eq!(RES, A::I(&X));
}

#[test]
fn enum_with_lifetimes_and_generics() {
    #[derive(Debug, PartialEq, New)]
    enum A<'a, T> {
        I { x: &'a T },
    }

    let x = 1u64;
    let res = A::new_i(&x);
    assert_eq!(res, A::I { x: &x });
}

#[test]
fn const_enum_with_lifetimes_and_generics() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A<'a, T> {
        I { x: &'a T },
    }

    const X: u64 = 1u64;
    const RES: A<u64> = A::new_i(&X);
    assert_eq!(RES, A::I { x: &X });
}

#[test]
fn tuple_enum_with_lifetimes_and_generics() {
    #[derive(Debug, PartialEq, New)]
    enum A<'a, T> {
        I(&'a T),
    }

    let x = 1u64;
    let res = A::new_i(&x);
    assert_eq!(res, A::I(&x));
}

#[test]
fn const_tuple_enum_with_lifetimes_and_generics() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A<'a, T> {
        I(&'a T),
    }

    const X: u64 = 1u64;
    const RES: A<u64> = A::new_i(&X);
    assert_eq!(RES, A::I(&X));
}

#[test]
fn enum_with_static_lifetime() {
    const X: &str = "abc";

    #[derive(Debug, PartialEq, New)]
    enum A {
        I { x: &'static str },
    }

    let res = A::new_i(X);
    assert_eq!(res, A::I { x: "abc" });
}

#[test]
fn const_enum_with_static_lifetime() {
    const X: &str = "abc";

    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I { x: &'static str },
    }

    const RES: A = A::new_i(X);
    assert_eq!(RES, A::I { x: "abc" });
}

#[test]
fn tuple_enum_with_static_lifetime() {
    const X: &str = "abc";

    #[derive(Debug, PartialEq, New)]
    enum A {
        I(&'static str),
    }

    let res = A::new_i(&X);
    assert_eq!(res, A::I("abc"));
}

#[test]
fn const_tuple_enum_with_static_lifetime() {
    const X: &str = "abc";

    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I(&'static str),
    }

    const RES: A = A::new_i(&X);
    assert_eq!(RES, A::I("abc"));
}

#[test]
fn enum_dyn_function() {
    #[derive(New)]
    enum A<'a> {
        I { f: &'a dyn Fn(f32) -> String },
    }

    let f = |x: f32| x.to_string();
    let res = A::new_i(&f);
    match res {
        A::I { f } => assert_eq!(f(3.14), "3.14"),
    }
}

#[test]
fn const_enum_dyn_function() {
    #[derive(New)]
    #[new(const)]
    enum A<'a> {
        I { f: &'a dyn Fn(f32) -> String },
    }

    const F: fn(f32) -> String = |x: f32| x.to_string();
    const RES: A = A::new_i(&F);
    match RES {
        A::I { f } => assert_eq!(f(3.14), "3.14"),
    }
}

#[test]
fn tuple_enum_dyn_function() {
    #[derive(New)]
    enum A<'a> {
        I(&'a dyn Fn(f32) -> String),
    }

    let f = |x: f32| x.to_string();
    let res = A::new_i(&f);
    match res {
        A::I(f) => assert_eq!(f(3.14), "3.14"),
    }
}

#[test]
fn const_tuple_enum_dyn_function() {
    #[derive(New)]
    #[new(const)]
    enum A<'a> {
        I(&'a dyn Fn(f32) -> String),
    }

    const F: fn(f32) -> String = |x: f32| x.to_string();
    const RES: A = A::new_i(&F);
    match RES {
        A::I(f) => assert_eq!(f(3.14), "3.14"),
    }
}
