use inew::New;
use std::marker::PhantomData;

#[test]
fn unit_like_struct_with_braces() {
    #[derive(Debug, PartialEq, New)]
    struct A {}

    let res = A::new();
    assert_eq!(res, A {});
}

#[test]
fn unit_like_struct_without_braces() {
    #[derive(Debug, PartialEq, New)]
    struct A;

    let res = A::new();
    assert_eq!(res, A {});
}

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
fn tuple_struct_single_field() {
    #[derive(New)]
    struct A(u32);

    let res = A::new(2);
    assert_eq!(res.0, 2);
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
fn tuple_struct_multiple_fields() {
    #[derive(New)]
    struct A(u32, u64);

    let res = A::new(2, 3);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 3);
}

#[test]
fn struct_type_alias() {
    type X = u32;

    #[derive(New)]
    struct A {
        x: X,
    }

    let res = A::new(2);
    assert_eq!(res.x, 2);
}

#[test]
fn tuple_struct_type_alias() {
    type X = u32;

    #[derive(New)]
    struct A(X);

    let res = A::new(2);
    assert_eq!(res.0, 2);
}

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
fn tuple_struct_unit_auto_default() {
    #[derive(New)]
    struct A(());

    let res = A::new();
    assert_eq!(res.0, ());
}

#[test]
fn struct_phantom_data_auto_default() {
    #[derive(New)]
    struct A<T> {
        x: PhantomData<T>,
    }

    let res: A<u32> = A::new();
    assert_eq!(res.x, PhantomData);
}

#[test]
fn tuple_struct_phantom_data_auto_default() {
    #[derive(New)]
    struct A<T>(PhantomData<T>);

    let res: A<u32> = A::new();
    assert_eq!(res.0, PhantomData);
}

#[test]
fn struct_with_default() {
    #[derive(New)]
    struct A {
        x: u32,
        #[new(default)]
        y: u64,
    }

    let res = A::new(2);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, 0);
}

#[test]
fn tuple_struct_with_default() {
    #[derive(New)]
    struct A(u32, #[new(default)] u64);

    let res = A::new(2);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 0);
}

#[test]
fn struct_with_default_expression() {
    #[derive(New)]
    struct A {
        x: u32,
        #[new(default = 1 + 2)]
        y: u64,
    }

    let res = A::new(2);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, 3);
}

#[test]
fn tuple_struct_with_default_expression() {
    #[derive(New)]
    struct A(u32, #[new(default = 1 + 2)] u64);

    let res = A::new(2);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 3);
}

#[test]
fn struct_with_default_custom_macro() {
    macro_rules! custom_macro {
        () => {
            7
        };
    }

    #[derive(New)]
    struct A {
        x: u32,
        #[new(default = custom_macro!())]
        y: u64,
    }

    let res = A::new(2);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, 7);
}

#[test]
fn tuple_struct_with_default_custom_macro() {
    macro_rules! custom_macro {
        () => {
            7
        };
    }

    #[derive(New)]
    struct A(u32, #[new(default = custom_macro!())] u64);

    let res = A::new(2);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 7);
}

#[test]
fn struct_with_default_allocation_macro() {
    #[derive(New)]
    struct A {
        x: u32,
        #[new(default = vec![ 1u32 ])]
        y: Vec<u32>,
    }

    let res = A::new(2);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, vec![1]);
}

#[test]
fn tuple_struct_with_default_allocation_macro() {
    #[derive(New)]
    struct A(u32, #[new(default = vec![ 1u32 ])] Vec<u32>);

    let res = A::new(2);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, vec![1]);
}

#[test]
fn struct_with_default_const_function() {
    const fn custom_default() -> u64 {
        3
    }

    #[derive(New)]
    struct A {
        x: u32,
        #[new(default = custom_default())]
        y: u64,
    }

    let res = A::new(2);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, 3);
}

#[test]
fn tuple_struct_with_default_const_function() {
    const fn custom_default() -> u64 {
        3
    }

    #[derive(New)]
    struct A(u32, #[new(default = custom_default())] u64);

    let res = A::new(2);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 3);
}

#[test]
fn struct_with_default_function() {
    fn custom_default() -> u64 {
        3
    }

    #[derive(New)]
    struct A {
        x: u32,
        #[new(default = custom_default())]
        y: u64,
    }

    let res = A::new(2);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, 3);
}

#[test]
fn tuple_struct_with_default_function() {
    fn custom_default() -> u64 {
        3
    }

    #[derive(New)]
    struct A(u32, #[new(default = custom_default())] u64);

    let res = A::new(2);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 3);
}

#[test]
fn struct_with_nested_default_const_function() {
    mod nested {
        pub const fn custom_default() -> u64 {
            3
        }
    }

    #[derive(New)]
    struct A {
        x: u32,
        #[new(default = nested::custom_default())]
        y: u64,
    }

    let res = A::new(2);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, 3);
}

#[test]
fn tuple_struct_with_nested_default_const_function() {
    mod nested {
        pub const fn custom_default() -> u64 {
            3
        }
    }

    #[derive(New)]
    struct A(u32, #[new(default = nested::custom_default())] u64);

    let res = A::new(2);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 3);
}

#[test]
fn struct_with_nested_default_function() {
    mod nested {
        pub fn custom_default() -> u64 {
            3
        }
    }

    #[derive(New)]
    struct A {
        x: u32,
        #[new(default = nested::custom_default())]
        y: u64,
    }

    let res = A::new(2);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, 3);
}

#[test]
fn tuple_struct_with_nested_default_function() {
    mod nested {
        pub fn custom_default() -> u64 {
            3
        }
    }

    #[derive(New)]
    struct A(u32, #[new(default = nested::custom_default())] u64);

    let res = A::new(2);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 3);
}

#[test]
fn struct_with_all_defaults() {
    #[derive(New)]
    struct A {
        #[new(default)]
        x: u32,
        #[new(default)]
        y: u64,
    }

    let res = A::new();
    assert_eq!(res.x, 0);
    assert_eq!(res.y, 0);
}

#[test]
fn tuple_struct_with_all_defaults() {
    #[derive(New)]
    struct A(#[new(default)] u32, #[new(default)] u64);

    let res = A::new();
    assert_eq!(res.0, 0);
    assert_eq!(res.1, 0);
}

#[test]
fn struct_with_mixed_all_defaults() {
    fn custom_default() -> u8 {
        5
    }

    #[derive(New)]
    struct A {
        #[new(default)]
        x: u32,
        #[new(default = 1 + 2)]
        y: u64,
        #[new(default = custom_default())]
        z: u8,
    }

    let res = A::new();
    assert_eq!(res.x, 0);
    assert_eq!(res.y, 3);
    assert_eq!(res.z, 5);
}

#[test]
fn tuple_struct_with_mixed_all_defaults() {
    fn custom_default() -> u8 {
        5
    }

    #[derive(New)]
    struct A(
        #[new(default)] u32,
        #[new(default = 1 + 2)] u64,
        #[new(default = custom_default())] u8,
    );

    let res = A::new();
    assert_eq!(res.0, 0);
    assert_eq!(res.1, 3);
    assert_eq!(res.2, 5);
}

#[test]
fn struct_into() {
    #[derive(New)]
    struct A {
        #[new(into)]
        x: String,
    }

    let res = A::new("abc");
    assert_eq!(res.x, "abc");
}

#[test]
fn tuple_struct_into() {
    #[derive(New)]
    struct A(#[new(into)] String);

    let res = A::new("abc");
    assert_eq!(res.0, "abc");
}

#[test]
fn struct_into_iter() {
    #[derive(New)]
    struct A {
        #[new(into_iter)]
        x: Vec<u32>,
    }

    let res = A::new(Some(5));
    assert_eq!(res.x, vec![5]);

    let res2 = A::new(None);
    assert_eq!(res2.x, vec![]);

    let res3 = A::new([1, 2, 3]);
    assert_eq!(res3.x, vec![1, 2, 3]);
}

#[test]
fn tuple_struct_into_iter() {
    #[derive(New)]
    struct A(#[new(into_iter)] Vec<u32>);

    let res = A::new(Some(5));
    assert_eq!(res.0, vec![5]);

    let res2 = A::new(None);
    assert_eq!(res2.0, vec![]);

    let res3 = A::new([1, 2, 3]);
    assert_eq!(res3.0, vec![1, 2, 3]);
}

#[test]
fn struct_alias_into_iter() {
    type X = Vec<u32>;

    #[derive(New)]
    struct A {
        #[new(into_iter = u32)]
        x: X,
    }

    let res = A::new(Some(5));
    assert_eq!(res.x, vec![5]);
}

#[test]
fn tuple_struct_alias_into_iter() {
    type X = Vec<u32>;

    #[derive(New)]
    struct A(#[new(into_iter = u32)] X);

    let res = A::new(Some(5));
    assert_eq!(res.0, vec![5]);
}

#[test]
fn struct_into_nested_iter() {
    #[derive(New)]
    struct A {
        #[new(into_iter)]
        x: Vec<Vec<u32>>,
    }

    let res = A::new(Some(vec![5]));
    assert_eq!(res.x, vec![vec![5]]);
}

#[test]
fn tuple_struct_into_nested_iter() {
    #[derive(New)]
    struct A(#[new(into_iter)] Vec<Vec<u32>>);

    let res = A::new(Some(vec![5]));
    assert_eq!(res.0, vec![vec![5]]);
}

#[test]
fn struct_alias_into_nested_iter() {
    type X = Vec<u32>;
    type Y = Vec<X>;

    #[derive(New)]
    struct A {
        #[new(into_iter = Vec<u32>)]
        y: Y,
    }

    let res = A::new(Some(vec![5]));
    assert_eq!(res.y, vec![vec![5]]);
}

#[test]
fn tuple_struct_alias_into_nested_iter() {
    type X = Vec<u32>;
    type Y = Vec<X>;

    #[derive(New)]
    struct A(#[new(into_iter = Vec<u32>)] Y);

    let res = A::new(Some(vec![5]));
    assert_eq!(res.0, vec![vec![5]]);
}

#[test]
fn struct_with_single_generic() {
    #[derive(New)]
    struct A<T> {
        x: T,
    }

    let res = A::new(1);
    assert_eq!(res.x, 1);
}

#[test]
fn tuple_struct_with_single_generic() {
    #[derive(New)]
    struct A<T>(T);

    let res = A::new(1);
    assert_eq!(res.0, 1);
}

#[test]
fn struct_with_single_generic_and_another_field() {
    #[derive(New)]
    struct A<T> {
        x: T,
        y: u32,
    }

    let res = A::new(1, 2);
    assert_eq!(res.x, 1);
    assert_eq!(res.y, 2);
}

#[test]
fn tuple_struct_with_single_generic_and_another_field() {
    #[derive(New)]
    struct A<T>(T, u32);

    let res = A::new(1, 2);
    assert_eq!(res.0, 1);
    assert_eq!(res.1, 2);
}

#[test]
fn struct_with_multiple_generics() {
    #[derive(New)]
    struct A<X, Y> {
        x: X,
        y: Y,
    }

    let res = A::new(1u32, 2u64);
    assert_eq!(res.x, 1);
    assert_eq!(res.y, 2);
}

#[test]
fn tuple_struct_with_multiple_generics() {
    #[derive(New)]
    struct A<X, Y>(X, Y);

    let res = A::new(1u32, 2u64);
    assert_eq!(res.0, 1);
    assert_eq!(res.1, 2);
}

#[test]
fn struct_with_nested_generics() {
    #[derive(New)]
    struct X<Y, Z> {
        y: Y,
        z: Z,
    }

    #[derive(New)]
    struct A<Y, Z> {
        x: X<Y, Z>,
    }

    let res = A::new(X::new(1, "z"));
    assert_eq!(res.x.y, 1);
    assert_eq!(res.x.z, "z");
}

#[test]
fn tuple_struct_with_nested_generics() {
    #[derive(New)]
    struct X<Y, Z>(Y, Z);

    #[derive(New)]
    struct A<Y, Z>(X<Y, Z>);

    let res = A::new(X::new(1, "z"));
    assert_eq!(res.0 .0, 1);
    assert_eq!(res.0 .1, "z");
}

#[test]
fn struct_with_default_generics() {
    #[derive(New)]
    struct A<X, Y: Default> {
        x: X,
        #[new(default)]
        y: Y,
    }

    let res = A::<u32, u64>::new(1u32);
    assert_eq!(res.x, 1);
    assert_eq!(res.y, 0);
}

#[test]
fn tuple_struct_with_default_generics() {
    #[derive(New)]
    struct A<X, Y: Default>(X, #[new(default)] Y);

    let res = A::<u32, u64>::new(1u32);
    assert_eq!(res.0, 1);
    assert_eq!(res.1, 0);
}

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
fn tuple_struct_with_lifetimes() {
    #[derive(New)]
    struct A<'a>(&'a u64);

    let x = 1u64;
    let res = A::new(&x);
    assert_eq!(*res.0, 1);
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
fn tuple_struct_with_lifetimes_and_generics() {
    #[derive(New)]
    struct A<'a, T>(&'a T);

    let x = 1u64;
    let res = A::new(&x);
    assert_eq!(*res.0, 1);
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
fn tuple_struct_with_static_lifetime() {
    const X: &str = "abc";

    #[derive(New)]
    struct A(&'static str);

    let res = A::new(&X);
    assert_eq!(res.0, "abc");
}

#[test]
fn struct_private_new() {
    #[derive(New)]
    #[new(pub = false)]
    struct A<'a, T> {
        x: &'a T,
    }

    let x = 1u64;
    let res = A::new(&x);
    assert_eq!(*res.x, 1);
}

#[test]
fn tuple_struct_private_new() {
    #[derive(New)]
    #[new(pub = false)]
    struct A<'a, T>(&'a T);

    let x = 1u64;
    let res = A::new(&x);
    assert_eq!(*res.0, 1);
}

#[test]
fn struct_rename_new() {
    #[derive(New)]
    #[new(rename = "create")]
    struct A<'a, T> {
        x: &'a T,
    }

    let x = 1u64;
    let res = A::create(&x);
    assert_eq!(*res.x, 1);
}

#[test]
fn tuple_struct_rename_new() {
    #[derive(New)]
    #[new(rename = "create")]
    struct A<'a, T>(&'a T);

    let x = 1u64;
    let res = A::create(&x);
    assert_eq!(*res.0, 1);
}
