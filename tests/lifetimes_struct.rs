#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};

use inew::New;

#[test]
fn struct_with_lifetimes() {
    #[derive(New)]
    struct A<'a> {
        x: &'a u64,
    }

    let x = 1u64;
    let res = A::new(&x);
    assert_eq!(*res.x, 1);
}

#[test]
fn const_struct_with_lifetimes() {
    #[derive(New)]
    #[new(const)]
    struct A<'a> {
        x: &'a u64,
    }

    const X: u64 = 1u64;
    const RES: A = A::new(&X);
    assert_eq!(*RES.x, 1);
}

#[test]
fn tuple_struct_with_lifetimes() {
    #[derive(New)]
    struct A<'a>(&'a u64);

    let x = 1u64;
    let res = A::new(&x);
    assert_eq!(*res.0, 1);
}

#[test]
fn const_tuple_struct_with_lifetimes() {
    #[derive(New)]
    #[new(const)]
    struct A<'a>(&'a u64);

    const X: u64 = 1u64;
    const RES: A = A::new(&X);
    assert_eq!(*RES.0, 1);
}

#[test]
fn struct_with_lifetimes_and_generics() {
    #[derive(New)]
    struct A<'a, T> {
        x: &'a T,
    }

    let x = 1u64;
    let res = A::new(&x);
    assert_eq!(*res.x, 1);
}

#[test]
fn const_struct_with_lifetimes_and_generics() {
    #[derive(New)]
    #[new(const)]
    struct A<'a, T> {
        x: &'a T,
    }

    const X: u64 = 1u64;
    const RES: A<u64> = A::new(&X);
    assert_eq!(*RES.x, 1);
}

#[test]
fn tuple_struct_with_lifetimes_and_generics() {
    #[derive(New)]
    struct A<'a, T>(&'a T);

    let x = 1u64;
    let res = A::new(&x);
    assert_eq!(*res.0, 1);
}

#[test]
fn const_tuple_struct_with_lifetimes_and_generics() {
    #[derive(New)]
    #[new(const)]
    struct A<'a, T>(&'a T);

    const X: u64 = 1u64;
    const RES: A<u64> = A::new(&X);
    assert_eq!(*RES.0, 1);
}

#[test]
fn struct_with_static_lifetime() {
    const X: &str = "abc";

    #[derive(New)]
    struct A {
        x: &'static str,
    }

    let res = A::new(X);
    assert_eq!(res.x, "abc");
}

#[test]
fn const_struct_with_static_lifetime() {
    const X: &str = "abc";

    #[derive(New)]
    #[new(const)]
    struct A {
        x: &'static str,
    }

    const RES: A = A::new(X);
    assert_eq!(RES.x, "abc");
}

#[test]
fn tuple_struct_with_static_lifetime() {
    const X: &str = "abc";

    #[derive(New)]
    struct A(&'static str);

    let res = A::new(&X);
    assert_eq!(res.0, "abc");
}

#[test]
fn const_tuple_struct_with_static_lifetime() {
    const X: &str = "abc";

    #[derive(New)]
    #[new(const)]
    struct A(&'static str);

    const RES: A = A::new(&X);
    assert_eq!(RES.0, "abc");
}

#[test]
fn struct_dyn_function() {
    #[derive(New)]
    struct A<'a> {
        f: &'a dyn Fn(f32) -> String,
    }

    let f = |x: f32| x.to_string();
    let res = A::new(&f);
    assert_eq!((res.f)(3.14), "3.14");
}

#[test]
fn const_struct_dyn_function() {
    #[derive(New)]
    #[new(const)]
    struct A<'a> {
        f: &'a dyn Fn(f32) -> String,
    }

    const F: fn(f32) -> String = |x: f32| x.to_string();
    const RES: A = A::new(&F);
    assert_eq!((RES.f)(3.14), "3.14");
}

#[test]
fn tuple_struct_dyn_function() {
    #[derive(New)]
    struct A<'a>(&'a dyn Fn(f32) -> String);

    let f = |x: f32| x.to_string();
    let res = A::new(&f);
    assert_eq!(res.0(3.14), "3.14");
}

#[test]
fn const_tuple_struct_dyn_function() {
    #[derive(New)]
    #[new(const)]
    struct A<'a>(&'a dyn Fn(f32) -> String);

    const F: fn(f32) -> String = |x: f32| x.to_string();
    const RES: A = A::new(&F);
    assert_eq!(RES.0(3.14), "3.14");
}
