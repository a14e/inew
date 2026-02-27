#![cfg_attr(not(feature = "std"), no_std)]

use inew::New;

#[test]
fn unit_like_struct_with_braces() {
    #[derive(Debug, PartialEq, New)]
    struct A {}

    let res = A::new();
    assert_eq!(res, A {});
}

#[test]
fn const_unit_like_struct_with_braces() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    struct A;

    const RES: A = A::new();
    assert_eq!(RES, A {});
}

#[test]
fn unit_like_struct_without_braces() {
    #[derive(Debug, PartialEq, New)]
    struct A;

    let res = A::new();
    assert_eq!(res, A {});
}

#[test]
fn const_unit_like_struct_without_braces() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    struct A;

    const RES: A = A::new();
    assert_eq!(RES, A {});
}
