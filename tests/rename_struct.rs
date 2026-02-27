#![cfg_attr(not(feature = "std"), no_std)]

use inew::New;

#[test]
fn struct_rename_new() {
    #[derive(New)]
    #[new(rename = "create")]
    struct A {
        x: u32,
    }

    let res = A::create(1);
    assert_eq!(res.x, 1);
}

#[test]
fn const_struct_rename_new() {
    #[derive(New)]
    #[new(rename = "create", const)]
    struct A {
        x: u32,
    }

    const RES: A = A::create(1);
    assert_eq!(RES.x, 1);
}

#[test]
fn tuple_struct_rename_new() {
    #[derive(New)]
    #[new(rename = "create")]
    struct A(u32);

    let res = A::create(1);
    assert_eq!(res.0, 1);
}

#[test]
fn const_tuple_struct_rename_new() {
    #[derive(New)]
    #[new(rename = "create", const)]
    struct A(u32);

    const RES: A = A::create(1);
    assert_eq!(RES.0, 1);
}
