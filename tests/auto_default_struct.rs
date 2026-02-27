#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use std::marker::PhantomData;
#[cfg(not(feature = "std"))]
use core::marker::PhantomData;

use inew::New;

#[test]
fn struct_unit_auto_default() {
    #[derive(New)]
    struct A {
        x: (),
    }

    let res = A::new();
    assert_eq!(res.x, ());
}

#[test]
fn const_struct_unit_auto_default() {
    #[derive(New)]
    #[new(const)]
    struct A {
        x: (),
    }

    const RES: A = A::new();
    assert_eq!(RES.x, ());
}

#[test]
fn tuple_struct_unit_auto_default() {
    #[derive(New)]
    struct A(());

    let res = A::new();
    assert_eq!(res.0, ());
}

#[test]
fn const_tuple_struct_unit_auto_default() {
    #[derive(New)]
    #[new(const)]
    struct A(());

    const RES: A = A::new();
    assert_eq!(RES.0, ());
}

#[test]
fn struct_phantom_data_auto_default() {
    #[derive(New)]
    struct A<T> {
        x: PhantomData<T>,
    }

    let res = A::<u32>::new();
    assert_eq!(res.x, PhantomData);
}

#[test]
fn const_struct_phantom_data_auto_default() {
    #[derive(New)]
    #[new(const)]
    struct A<T> {
        x: PhantomData<T>,
    }

    const RES: A<u32> = A::new();
    assert_eq!(RES.x, PhantomData);
}

#[test]
fn tuple_struct_phantom_data_auto_default() {
    #[derive(New)]
    struct A<T>(PhantomData<T>);

    let res = A::<u32>::new();
    assert_eq!(res.0, PhantomData);
}

#[test]
fn const_tuple_struct_phantom_data_auto_default() {
    #[derive(New)]
    #[new(const)]
    struct A<T>(PhantomData<T>);

    const RES: A<u32> = A::new();
    assert_eq!(RES.0, PhantomData);
}

#[test]
fn struct_core_phantom_data_auto_default() {
    #[derive(New)]
    struct A<T> {
        x: core::marker::PhantomData<T>,
    }

    let res = A::<u32>::new();
    assert_eq!(res.x, PhantomData);
}

#[test]
fn const_struct_core_phantom_data_auto_default() {
    #[derive(New)]
    #[new(const)]
    struct A<T> {
        x: core::marker::PhantomData<T>,
    }

    const RES: A<u32> = A::new();
    assert_eq!(RES.x, PhantomData);
}

#[test]
fn tuple_struct_core_phantom_data_auto_default() {
    #[derive(New)]
    struct A<T>(core::marker::PhantomData<T>);

    let res = A::<u32>::new();
    assert_eq!(res.0, PhantomData);
}

#[test]
fn const_tuple_struct_core_phantom_data_auto_default() {
    #[derive(New)]
    #[new(const)]
    struct A<T>(core::marker::PhantomData<T>);

    const RES: A<u32> = A::new();
    assert_eq!(RES.0, PhantomData);
}

#[cfg(feature = "std")]
#[test]
fn struct_std_phantom_data_auto_default() {
    #[derive(New)]
    struct A<T> {
        x: std::marker::PhantomData<T>,
    }

    let res = A::<u32>::new();
    assert_eq!(res.x, PhantomData);
}

#[cfg(feature = "std")]
#[test]
fn const_struct_std_phantom_data_auto_default() {
    #[derive(New)]
    #[new(const)]
    struct A<T> {
        x: std::marker::PhantomData<T>,
    }

    const RES: A<u32> = A::new();
    assert_eq!(RES.x, PhantomData);
}

#[cfg(feature = "std")]
#[test]
fn tuple_struct_std_phantom_data_auto_default() {
    #[derive(New)]
    struct A<T>(std::marker::PhantomData<T>);

    let res = A::<u32>::new();
    assert_eq!(res.0, PhantomData);
}

#[cfg(feature = "std")]
#[test]
fn const_tuple_struct_std_phantom_data_auto_default() {
    #[derive(New)]
    #[new(const)]
    struct A<T>(std::marker::PhantomData<T>);

    const RES: A<u32> = A::new();
    assert_eq!(RES.0, PhantomData);
}
