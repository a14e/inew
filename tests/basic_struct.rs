#![cfg_attr(not(feature = "std"), no_std)]

use inew::New;

#[test]
fn struct_single_field() {
    #[derive(New)]
    struct A {
        x: u32,
    }

    let res = A::new(1);
    assert_eq!(res.x, 1)
}

#[test]
fn const_struct_single_field() {
    #[derive(New)]
    #[new(const)]
    struct A {
        x: u32,
    }

    const RES: A = A::new(1);
    assert_eq!(RES.x, 1);
}

#[test]
fn tuple_struct_single_field() {
    #[derive(New)]
    struct A(u32);

    let res = A::new(2);
    assert_eq!(res.0, 2);
}

#[test]
fn const_tuple_struct_single_field() {
    #[derive(New)]
    #[new(const)]
    struct A(u32);

    const RES: A = A::new(2);
    assert_eq!(RES.0, 2);
}

#[test]
fn struct_explicit_const() {
    #[derive(New)]
    #[new(const = false)]
    struct A {
        x: u32,
    }

    let res = A::new(1);
    assert_eq!(res.x, 1);
}

#[test]
fn const_struct_explicit_const() {
    #[derive(New)]
    #[new(const = true)]
    struct A {
        x: u32,
    }

    const RES: A = A::new(1);
    assert_eq!(RES.x, 1);
}

#[test]
fn tuple_struct_explicit_const() {
    #[derive(New)]
    #[new(const = false)]
    struct A(u32);

    let res = A::new(2);
    assert_eq!(res.0, 2);
}

#[test]
fn const_tuple_struct_explicit_const() {
    #[derive(New)]
    #[new(const = true)]
    struct A(u32);

    const RES: A = A::new(2);
    assert_eq!(RES.0, 2);
}

#[test]
fn struct_multiple_fields() {
    #[derive(New)]
    struct A {
        x: u32,
        y: u64,
    }

    let res = A::new(2, 3);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, 3);
}

#[test]
fn const_struct_multiple_fields() {
    #[derive(New)]
    #[new(const)]
    struct A {
        x: u32,
        y: u64,
    }

    const RES: A = A::new(2, 3);
    assert_eq!(RES.x, 2);
    assert_eq!(RES.y, 3);
}

#[test]
fn tuple_struct_multiple_fields() {
    #[derive(New)]
    struct A(u32, u64);

    let res = A::new(2, 3);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 3);
}

#[test]
fn const_tuple_struct_multiple_fields() {
    #[derive(New)]
    #[new(const)]
    struct A(u32, u64);

    const RES: A = A::new(2, 3);
    assert_eq!(RES.0, 2);
    assert_eq!(RES.1, 3);
}
