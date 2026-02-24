use inew::New;
use std::marker::PhantomData;

#[test]
fn const_unit_like_struct_with_braces() {
    #[derive(Debug, PartialEq, New)]
    #[new(const = true)]
    struct A;

    const RES: A = A::new();
    assert_eq!(RES, A {});
}

#[test]
fn const_unit_like_struct_without_braces() {
    #[derive(Debug, PartialEq, New)]
    #[new(const = true)]
    struct A;

    const RES: A = A::new();
    assert_eq!(RES, A {});
}

#[test]
fn const_struct_single_field() {
    #[derive(New)]
    #[new(const = true)]
    struct A {
        x: u32,
    }

    const RES: A = A::new(1);
    assert_eq!(RES.x, 1);
}

#[test]
fn const_tuple_struct_single_field() {
    #[derive(New)]
    #[new(const = true)]
    struct A(u32);

    const RES: A = A::new(2);
    assert_eq!(RES.0, 2);
}

#[test]
fn const_struct_multiple_fields() {
    #[derive(New)]
    #[new(const = true)]
    struct A {
        x: u32,
        y: u64,
    }

    const RES: A = A::new(2, 3);
    assert_eq!(RES.x, 2);
    assert_eq!(RES.y, 3);
}

#[test]
fn const_tuple_struct_multiple_fields() {
    #[derive(New)]
    #[new(const = true)]
    struct A(u32, u64);

    const RES: A = A::new(2, 3);
    assert_eq!(RES.0, 2);
    assert_eq!(RES.1, 3);
}

#[test]
fn const_struct_type_alias() {
    type X = u32;

    #[derive(New)]
    #[new(const = true)]
    struct A {
        x: X,
    }

    const RES: A = A::new(2);
    assert_eq!(RES.x, 2);
}

#[test]
fn const_tuple_struct_type_alias() {
    type X = u32;

    #[derive(New)]
    #[new(const = true)]
    struct A(X);

    const RES: A = A::new(2);
    assert_eq!(RES.0, 2);
}

#[test]
fn const_struct_unit_auto_default() {
    #[derive(New)]
    #[new(const = true)]
    struct A {
        x: (),
    }

    const RES: A = A::new();
    assert_eq!(RES.x, ());
}

#[test]
fn const_tuple_struct_unit_auto_default() {
    #[derive(New)]
    #[new(const = true)]
    struct A(());

    const RES: A = A::new();
    assert_eq!(RES.0, ());
}

#[test]
fn const_struct_phantom_data_auto_default() {
    #[derive(New)]
    #[new(const = true)]
    struct A<T> {
        x: PhantomData<T>,
    }

    const RES: A<u32> = A::new();
    assert_eq!(RES.x, PhantomData);
}

#[test]
fn const_tuple_struct_phantom_data_auto_default() {
    #[derive(New)]
    #[new(const = true)]
    struct A<T>(PhantomData<T>);

    const RES: A<u32> = A::new();
    assert_eq!(RES.0, PhantomData);
}

#[test]
fn const_struct_with_default_expression() {
    #[derive(New)]
    #[new(const = true)]
    struct A {
        x: u32,
        #[new(default = 1 + 2)]
        y: u64,
    }

    const RES: A = A::new(2);
    assert_eq!(RES.x, 2);
    assert_eq!(RES.y, 3);
}

#[test]
fn const_tuple_struct_with_default_expression() {
    #[derive(New)]
    #[new(const = true)]
    struct A(u32, #[new(default = 1 + 2)] u64);

    const RES: A = A::new(2);
    assert_eq!(RES.0, 2);
    assert_eq!(RES.1, 3);
}

#[test]
fn const_struct_with_default_custom_macro() {
    macro_rules! custom_macro {
        () => {
            7
        };
    }

    #[derive(New)]
    #[new(const = true)]
    struct A {
        x: u32,
        #[new(default = custom_macro!())]
        y: u64,
    }

    const RES: A = A::new(2);
    assert_eq!(RES.x, 2);
    assert_eq!(RES.y, 7);
}

#[test]
fn const_tuple_struct_with_default_custom_macro() {
    macro_rules! custom_macro {
        () => {
            7
        };
    }

    #[derive(New)]
    #[new(const = true)]
    struct A(u32, #[new(default = custom_macro!())] u64);

    const RES: A = A::new(2);
    assert_eq!(RES.0, 2);
    assert_eq!(RES.1, 7);
}

#[test]
fn const_struct_with_default_const_function() {
    const fn custom_default() -> u64 {
        3
    }

    #[derive(New)]
    #[new(const = true)]
    struct A {
        x: u32,
        #[new(default = custom_default())]
        y: u64,
    }

    const RES: A = A::new(2);
    assert_eq!(RES.x, 2);
    assert_eq!(RES.y, 3);
}

#[test]
fn const_tuple_struct_with_default_const_function() {
    const fn custom_default() -> u64 {
        3
    }

    #[derive(New)]
    #[new(const = true)]
    struct A(u32, #[new(default = custom_default())] u64);

    const RES: A = A::new(2);
    assert_eq!(RES.0, 2);
    assert_eq!(RES.1, 3);
}

#[test]
fn const_struct_with_nested_default_const_function() {
    mod nested {
        pub const fn custom_default() -> u64 {
            3
        }
    }

    #[derive(New)]
    #[new(const = true)]
    struct A {
        x: u32,
        #[new(default = nested::custom_default())]
        y: u64,
    }

    const RES: A = A::new(2);
    assert_eq!(RES.x, 2);
    assert_eq!(RES.y, 3);
}

#[test]
fn const_tuple_struct_with_nested_default_const_function() {
    mod nested {
        pub const fn custom_default() -> u64 {
            3
        }
    }

    #[derive(New)]
    #[new(const = true)]
    struct A(u32, #[new(default = nested::custom_default())] u64);

    const RES: A = A::new(2);
    assert_eq!(RES.0, 2);
    assert_eq!(RES.1, 3);
}

#[test]
fn const_struct_with_single_generic() {
    #[derive(New)]
    #[new(const = true)]
    struct A<T> {
        x: T,
    }

    const RES: A<u32> = A::new(1);
    assert_eq!(RES.x, 1);
}

#[test]
fn const_tuple_struct_with_single_generic() {
    #[derive(New)]
    #[new(const = true)]
    struct A<T>(T);

    const RES: A<u32> = A::new(1);
    assert_eq!(RES.0, 1);
}

#[test]
fn const_struct_with_single_generic_and_another_field() {
    #[derive(New)]
    #[new(const = true)]
    struct A<T> {
        x: T,
        y: u32,
    }

    const RES: A<u32> = A::new(1, 2);
    assert_eq!(RES.x, 1);
    assert_eq!(RES.y, 2);
}

#[test]
fn const_tuple_struct_with_single_generic_and_another_field() {
    #[derive(New)]
    #[new(const = true)]
    struct A<T>(T, u32);

    const RES: A<u32> = A::new(1, 2);
    assert_eq!(RES.0, 1);
    assert_eq!(RES.1, 2);
}

#[test]
fn const_struct_with_multiple_generics() {
    #[derive(New)]
    #[new(const = true)]
    struct A<X, Y> {
        x: X,
        y: Y,
    }

    const RES: A<u32, u64> = A::new(1u32, 2u64);
    assert_eq!(RES.x, 1);
    assert_eq!(RES.y, 2);
}

#[test]
fn const_tuple_struct_with_multiple_generics() {
    #[derive(New)]
    #[new(const = true)]
    struct A<X, Y>(X, Y);

    const RES: A<u32, u64> = A::new(1u32, 2u64);
    assert_eq!(RES.0, 1);
    assert_eq!(RES.1, 2);
}

#[test]
fn const_struct_with_nested_generics() {
    #[derive(New)]
    #[new(const = true)]
    struct X<Y, Z> {
        y: Y,
        z: Z,
    }

    #[derive(New)]
    #[new(const = true)]
    struct A<Y, Z> {
        x: X<Y, Z>,
    }

    const RES: A<u32, &str> = A::new(X::new(1, "z"));
    assert_eq!(RES.x.y, 1);
    assert_eq!(RES.x.z, "z");
}

#[test]
fn const_tuple_struct_with_nested_generics() {
    #[derive(New)]
    #[new(const = true)]
    struct X<Y, Z>(Y, Z);

    #[derive(New)]
    #[new(const = true)]
    struct A<Y, Z>(X<Y, Z>);

    const RES: A<u32, &str> = A::new(X::new(1, "z"));
    assert_eq!(RES.0 .0, 1);
    assert_eq!(RES.0 .1, "z");
}

#[test]
fn const_struct_with_lifetimes() {
    #[derive(New)]
    #[new(const = true)]
    struct A<'a> {
        x: &'a u64,
    }

    const X: u64 = 1u64;
    const RES: A = A::new(&X);
    assert_eq!(*RES.x, 1);
}

#[test]
fn const_tuple_struct_with_lifetimes() {
    #[derive(New)]
    #[new(const = true)]
    struct A<'a>(&'a u64);

    const X: u64 = 1u64;
    const RES: A = A::new(&X);
    assert_eq!(*RES.0, 1);
}

#[test]
fn const_struct_with_lifetimes_and_generics() {
    #[derive(New)]
    #[new(const = true)]
    struct A<'a, T> {
        x: &'a T,
    }

    const X: u64 = 1u64;
    const RES: A<u64> = A::new(&X);
    assert_eq!(*RES.x, 1);
}

#[test]
fn const_tuple_struct_with_lifetimes_and_generics() {
    #[derive(New)]
    #[new(const = true)]
    struct A<'a, T>(&'a T);

    const X: u64 = 1u64;
    const RES: A<u64> = A::new(&X);
    assert_eq!(*RES.0, 1);
}

#[test]
fn const_struct_with_static_lifetime() {
    const X: &str = "abc";

    #[derive(New)]
    #[new(const = true)]
    struct A {
        x: &'static str,
    }

    const RES: A = A::new(X);
    assert_eq!(RES.x, "abc");
}

#[test]
fn const_tuple_struct_with_static_lifetime() {
    const X: &str = "abc";

    #[derive(New)]
    #[new(const = true)]
    struct A(&'static str);

    const RES: A = A::new(&X);
    assert_eq!(RES.0, "abc");
}

#[test]
fn const_struct_private_new() {
    #[derive(New)]
    #[new(pub = false, const = true)]
    struct A {
        x: u32,
    }

    const RES: A = A::new(1);
    assert_eq!(RES.x, 1);
}

#[test]
fn const_tuple_struct_private_new() {
    #[derive(New)]
    #[new(pub = false, const = true)]
    struct A(u32);

    const RES: A = A::new(1);
    assert_eq!(RES.0, 1);
}


#[test]
fn const_struct_custom_visibility() {
    #[derive(New)]
    #[new(pub = "crate", const = true)]
    struct A {
        x: u32,
    }

    mod nested {
        use super::*;

        #[derive(New)]
        #[new(pub = "super", const = true)]
        pub struct B {
            pub x: u32,
        }
    }

    #[derive(New)]
    #[new(pub = "self", const = true)]
    struct C {
        x: u32,
    }

    const RES: A = A::new(1);
    assert_eq!(RES.x, 1);

    const RES2: nested::B = nested::B::new(2);
    assert_eq!(RES2.x, 2);

    const RES3: C = C::new(3);
    assert_eq!(RES3.x, 3);
}

#[test]
fn const_tuple_struct_custom_visibility() {
    #[derive(New)]
    #[new(pub = "crate", const = true)]
    struct A(u32);

    mod nested {
        use super::*;

        #[derive(New)]
        #[new(pub = "super", const = true)]
        pub struct B(pub u32);
    }

    #[derive(New)]
    #[new(pub = "self", const = true)]
    struct C(u32);

    const RES: A = A::new(1);
    assert_eq!(RES.0, 1);

    const RES2: nested::B = nested::B::new(2);
    assert_eq!(RES2.0, 2);

    const RES3: C = C::new(3);
    assert_eq!(RES3.0, 3);
}

#[test]
fn const_struct_rename_new() {
    #[derive(New)]
    #[new(rename = "create", const = true)]
    struct A {
        x: u32,
    }

    const RES: A = A::create(1);
    assert_eq!(RES.x, 1);
}

#[test]
fn const_tuple_struct_rename_new() {
    #[derive(New)]
    #[new(rename = "create", const = true)]
    struct A(u32);

    const RES: A = A::create(1);
    assert_eq!(RES.0, 1);
}
