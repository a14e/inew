use inew::New;
use std::marker::PhantomData;

#[test]
fn unit_like_struct_with_braces() {
    #[derive(Debug, PartialEq, New)]
    #[new(const = true)]
    struct A;

    const RES: A = A::new();
    assert_eq!(RES, A {});
}

#[test]
fn unit_like_struct_without_braces() {
    #[derive(Debug, PartialEq, New)]
    #[new(const = true)]
    struct A;

    const RES: A = A::new();
    assert_eq!(RES, A {});
}

#[test]
fn struct_single_field() {
    #[derive(New)]
    #[new(const = true)]
    struct A {
        x: u32,
    }

    const RES: A = A::new(1);
    assert_eq!(RES.x, 1);
}

#[test]
fn tuple_struct_single_field() {
    #[derive(New)]
    #[new(const = true)]
    struct A(u32);

    const RES: A = A::new(2);
    assert_eq!(RES.0, 2);
}

#[test]
fn struct_multiple_fields() {
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
fn tuple_struct_multiple_fields() {
    #[derive(New)]
    #[new(const = true)]
    struct A(u32, u64);

    const RES: A = A::new(2, 3);
    assert_eq!(RES.0, 2);
    assert_eq!(RES.1, 3);
}

#[test]
fn struct_type_alias() {
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
fn tuple_struct_type_alias() {
    type X = u32;

    #[derive(New)]
    #[new(const = true)]
    struct A(X);

    const RES: A = A::new(2);
    assert_eq!(RES.0, 2);
}

#[test]
fn struct_unit_auto_default() {
    #[derive(New)]
    #[new(const = true)]
    struct A {
        x: (),
    }

    const RES: A = A::new();
    assert_eq!(RES.x, ());
}

#[test]
fn tuple_struct_unit_auto_default() {
    #[derive(New)]
    #[new(const = true)]
    struct A(());

    const RES: A = A::new();
    assert_eq!(RES.0, ());
}

#[test]
fn struct_phantom_data_auto_default() {
    #[derive(New)]
    #[new(const = true)]
    struct A<T> {
        x: PhantomData<T>,
    }

    const RES: A<u32> = A::new();
    assert_eq!(RES.x, PhantomData);
}

#[test]
fn tuple_struct_phantom_data_auto_default() {
    #[derive(New)]
    #[new(const = true)]
    struct A<T>(PhantomData<T>);

    const RES: A<u32> = A::new();
    assert_eq!(RES.0, PhantomData);
}

#[test]
fn struct_with_default_expression() {
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
fn tuple_struct_with_default_expression() {
    #[derive(New)]
    #[new(const = true)]
    struct A(u32, #[new(default = 1 + 2)] u64);

    const RES: A = A::new(2);
    assert_eq!(RES.0, 2);
    assert_eq!(RES.1, 3);
}

#[test]
fn struct_with_default_custom_macro() {
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
fn tuple_struct_with_default_custom_macro() {
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
fn struct_with_default_const_function() {
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
fn tuple_struct_with_default_const_function() {
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
fn struct_with_nested_default_const_function() {
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
fn tuple_struct_with_nested_default_const_function() {
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
fn struct_with_single_generic() {
    #[derive(New)]
    #[new(const = true)]
    struct A<T> {
        x: T,
    }

    const RES: A<u32> = A::new(1);
    assert_eq!(RES.x, 1);
}

#[test]
fn tuple_struct_with_single_generic() {
    #[derive(New)]
    #[new(const = true)]
    struct A<T>(T);

    const RES: A<u32> = A::new(1);
    assert_eq!(RES.0, 1);
}

#[test]
fn struct_with_single_generic_and_another_field() {
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
fn tuple_struct_with_single_generic_and_another_field() {
    #[derive(New)]
    #[new(const = true)]
    struct A<T>(T, u32);

    const RES: A<u32> = A::new(1, 2);
    assert_eq!(RES.0, 1);
    assert_eq!(RES.1, 2);
}

#[test]
fn struct_with_multiple_generics() {
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
fn tuple_struct_with_multiple_generics() {
    #[derive(New)]
    #[new(const = true)]
    struct A<X, Y>(X, Y);

    const RES: A<u32, u64> = A::new(1u32, 2u64);
    assert_eq!(RES.0, 1);
    assert_eq!(RES.1, 2);
}

#[test]
fn struct_with_nested_generics() {
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
fn tuple_struct_with_nested_generics() {
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
fn struct_with_lifetimes() {
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
fn tuple_struct_with_lifetimes() {
    #[derive(New)]
    #[new(const = true)]
    struct A<'a>(&'a u64);

    const X: u64 = 1u64;
    const RES: A = A::new(&X);
    assert_eq!(*RES.0, 1);
}

#[test]
fn struct_with_lifetimes_and_generics() {
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
fn tuple_struct_with_lifetimes_and_generics() {
    #[derive(New)]
    #[new(const = true)]
    struct A<'a, T>(&'a T);

    const X: u64 = 1u64;
    const RES: A<u64> = A::new(&X);
    assert_eq!(*RES.0, 1);
}

#[test]
fn struct_with_static_lifetime() {
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
fn tuple_struct_with_static_lifetime() {
    const X: &str = "abc";

    #[derive(New)]
    #[new(const = true)]
    struct A(&'static str);

    const RES: A = A::new(&X);
    assert_eq!(RES.0, "abc");
}

#[test]
fn struct_private_new() {
    #[derive(New)]
    #[new(pub = false, const = true)]
    struct A<'a, T> {
        x: &'a T,
    }

    const X: u64 = 1u64;
    const RES: A<u64> = A::new(&X);
    assert_eq!(*RES.x, 1);
}

#[test]
fn tuple_struct_private_new() {
    #[derive(New)]
    #[new(pub = false, const = true)]
    struct A<'a, T>(&'a T);

    const X: u64 = 1u64;
    const RES: A<u64> = A::new(&X);
    assert_eq!(*RES.0, 1);
}

#[test]
fn struct_rename_new() {
    #[derive(New)]
    #[new(rename = "create", const = true)]
    struct A<'a, T> {
        x: &'a T,
    }

    const X: u64 = 1u64;
    const RES: A<u64> = A::create(&X);
    assert_eq!(*RES.x, 1);
}

#[test]
fn tuple_struct_rename_new() {
    #[derive(New)]
    #[new(rename = "create", const = true)]
    struct A<'a, T>(&'a T);

    const X: u64 = 1u64;
    const RES: A<u64> = A::create(&X);
    assert_eq!(*RES.0, 1);
}
