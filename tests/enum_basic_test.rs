use inew::New;
use std::marker::PhantomData;

#[test]
fn unit_like_enum_with_braces() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I {},
    }

    let res = A::new_i();
    assert_eq!(res, A::I {});
}

#[test]
fn unit_like_enum_without_braces() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I,
    }

    let res = A::new_i();
    assert_eq!(res, A::I);
}

#[test]
fn unit_like_enum_with_parentheses() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(),
    }

    let res = A::new_i();
    assert_eq!(res, A::I());
}

#[test]
fn enum_single_field() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I { x: u32 },
    }

    let res = A::new_i(1);
    assert_eq!(res, A::I { x: 1 });
}

#[test]
fn tuple_enum_single_field() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(u32),
    }

    let res = A::new_i(1);
    assert_eq!(res, A::I(1));
}

#[test]
fn enum_multiple_fields() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I { x: u32, y: u64 },
    }

    let res = A::new_i(2, 3);
    assert_eq!(res, A::I { x: 2, y: 3 });
}

#[test]
fn tuple_enum_multiple_fields() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(u32, u64),
    }

    let res = A::new_i(2, 3);
    assert_eq!(res, A::I(2, 3));
}

#[test]
fn enum_type_alias() {
    type X = u32;

    #[derive(Debug, PartialEq, New)]
    enum A {
        I { x: X },
    }

    let res = A::new_i(2);
    assert_eq!(res, A::I { x: 2 });
}

#[test]
fn tuple_enum_type_alias() {
    type X = u32;

    #[derive(Debug, PartialEq, New)]
    enum A {
        I(X),
    }

    let res = A::new_i(2);
    assert_eq!(res, A::I(2));
}

#[test]
fn enum_mixed_variants() {
    #[derive(Debug, PartialEq, New)]
    enum MyEnum {
        I,
        J { x: u32 },
        K (u64, u64),
        L { x: u8, y: u8 },
        M(u16, u16, u16, bool),
    }

    let res = MyEnum::new_i();
    assert_eq!(res, MyEnum::I);

    let res2 = MyEnum::new_j(1);
    assert_eq!(res2, MyEnum::J { x: 1 });

    let res3 = MyEnum::new_k(1, 2);
    assert_eq!(res3, MyEnum::K (1, 2));

    let res4 = MyEnum::new_l(1, 2);
    assert_eq!(res4, MyEnum::L { x: 1, y: 2 });

    let res5 = MyEnum::new_m(1, 2, 3, true);
    assert_eq!(res5, MyEnum::M(1, 2, 3, true));
}

#[test]
fn enum_unit_auto_default() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I { x: () },
    }

    let res = A::new_i();
    assert_eq!(res, A::I { x: () });
}

#[test]
fn tuple_enum_unit_auto_default() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(()),
    }

    let res = A::new_i();
    assert_eq!(res, A::I(()));
}

#[test]
fn enum_phantom_data_auto_default() {
    #[derive(Debug, PartialEq, New)]
    enum A<T> {
        I { x: PhantomData<T> },
    }

    let res: A<u32> = A::new_i();
    assert_eq!(res, A::I { x: PhantomData });
}

#[test]
fn tuple_enum_phantom_data_auto_default() {
    #[derive(Debug, PartialEq, New)]
    enum A<T> {
        I(PhantomData<T>),
    }

    let res: A<u32> = A::new_i();
    assert_eq!(res, A::I(PhantomData));
}

#[test]
fn enum_with_default() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            x: u32,
            #[new(default)]
            y: u64,
        },
    }

    let res = A::new_i(2);
    assert_eq!(res, A::I { x: 2, y: 0 });
}

#[test]
fn tuple_enum_with_default() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(u32, #[new(default)] u64),
    }

    let res = A::new_i(2);
    assert_eq!(res, A::I(2, 0));
}

#[test]
fn enum_with_default_expression() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            x: u32,
            #[new(default = 1 + 2)]
            y: u64,
        },
    }

    let res = A::new_i(2);
    assert_eq!(res, A::I { x: 2, y: 3 });
}

#[test]
fn tuple_enum_with_default_expression() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(u32, #[new(default = 1 + 2)] u64),
    }

    let res = A::new_i(2);
    assert_eq!(res, A::I(2, 3));
}

#[test]
fn enum_with_default_custom_macro() {
    macro_rules! custom_macro {
        () => {
            7
        };
    }

    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            x: u32,
            #[new(default = custom_macro!())]
            y: u64,
        },
    }

    let res = A::new_i(2);
    assert_eq!(res, A::I { x: 2, y: 7 });
}

#[test]
fn tuple_enum_with_default_custom_macro() {
    macro_rules! custom_macro {
        () => {
            7
        };
    }

    #[derive(Debug, PartialEq, New)]
    enum A {
        I(u32, #[new(default = custom_macro!())] u64),
    }

    let res = A::new_i(2);
    assert_eq!(res, A::I(2, 7));
}

#[test]
fn enum_with_default_allocation_macro() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            x: u32,
            #[new(default = vec![ 1u32 ])]
            y: Vec<u32>,
        },
    }

    let res = A::new_i(2);
    assert_eq!(res, A::I { x: 2, y: vec![1] });
}

#[test]
fn tuple_enum_with_default_allocation_macro() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(u32, #[new(default = vec![ 1u32 ])] Vec<u32>),
    }

    let res = A::new_i(2);
    assert_eq!(res, A::I(2, vec![1]));
}

#[test]
fn enum_with_default_const_function() {
    const fn custom_default() -> u64 {
        3
    }

    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            x: u32,
            #[new(default = custom_default())]
            y: u64,
        },
    }

    let res = A::new_i(2);
    assert_eq!(res, A::I { x: 2, y: 3 });
}

#[test]
fn tuple_enum_with_default_const_function() {
    const fn custom_default() -> u64 {
        3
    }

    #[derive(Debug, PartialEq, New)]
    enum A {
        I(u32, #[new(default = custom_default())] u64),
    }

    let res = A::new_i(2);
    assert_eq!(res, A::I(2, 3));
}

#[test]
fn enum_with_default_function() {
    fn custom_default() -> u64 {
        3
    }

    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            x: u32,
            #[new(default = custom_default())]
            y: u64,
        },
    }

    let res = A::new_i(2);
    assert_eq!(res, A::I { x: 2, y: 3 });
}

#[test]
fn tuple_enum_with_default_function() {
    fn custom_default() -> u64 {
        3
    }

    #[derive(Debug, PartialEq, New)]
    enum A {
        I(u32, #[new(default = custom_default())] u64),
    }

    let res = A::new_i(2);
    assert_eq!(res, A::I(2, 3));
}

#[test]
fn enum_with_nested_default_const_function() {
    mod nested {
        pub const fn custom_default() -> u64 {
            3
        }
    }

    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            x: u32,
            #[new(default = nested::custom_default())]
            y: u64,
        },
    }

    let res = A::new_i(2);
    assert_eq!(res, A::I { x: 2, y: 3 });
}

#[test]
fn tuple_enum_with_nested_default_const_function() {
    mod nested {
        pub const fn custom_default() -> u64 {
            3
        }
    }

    #[derive(Debug, PartialEq, New)]
    enum A {
        I(u32, #[new(default = nested::custom_default())] u64),
    }

    let res = A::new_i(2);
    assert_eq!(res, A::I(2, 3));
}

#[test]
fn enum_with_nested_default_function() {
    mod nested {
        pub fn custom_default() -> u64 {
            3
        }
    }

    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            x: u32,
            #[new(default = nested::custom_default())]
            y: u64,
        },
    }

    let res = A::new_i(2);
    assert_eq!(res, A::I { x: 2, y: 3 });
}

#[test]
fn tuple_enum_with_nested_default_function() {
    mod nested {
        pub fn custom_default() -> u64 {
            3
        }
    }

    #[derive(Debug, PartialEq, New)]
    enum A {
        I(u32, #[new(default = nested::custom_default())] u64),
    }

    let res = A::new_i(2);
    assert_eq!(res, A::I(2, 3));
}

#[test]
fn enum_with_all_defaults() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            #[new(default)]
            x: u32,
            #[new(default)]
            y: u64,
        },
    }

    let res = A::new_i();
    assert_eq!(res, A::I { x: 0, y: 0 });
}

#[test]
fn tuple_enum_with_all_defaults() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(#[new(default)] u32, #[new(default)] u64),
    }

    let res = A::new_i();
    assert_eq!(res, A::I(0, 0));
}

#[test]
fn enum_with_mixed_all_defaults() {
    fn custom_default() -> u8 {
        5
    }

    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            #[new(default)]
            x: u32,
            #[new(default = 1 + 2)]
            y: u64,
            #[new(default = custom_default())]
            z: u8,
        },
    }

    let res = A::new_i();
    assert_eq!(res, A::I { x: 0, y: 3, z: 5 });
}

#[test]
fn tuple_enum_with_mixed_all_defaults() {
    fn custom_default() -> u8 {
        5
    }

    #[derive(Debug, PartialEq, New)]
    enum A {
        I(
            #[new(default)] u32,
            #[new(default = 1 + 2)] u64,
            #[new(default = custom_default())] u8,
        ),
    }

    let res = A::new_i();
    assert_eq!(res, A::I(0, 3, 5));
}

#[test]
fn enum_into() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            #[new(into)]
            x: String,
        },
    }

    let res = A::new_i("abc");
    assert_eq!(
        res,
        A::I {
            x: "abc".to_string()
        }
    );
}

#[test]
fn tuple_enum_into() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(#[new(into)] String),
    }

    let res = A::new_i("abc");
    assert_eq!(res, A::I("abc".to_string()));
}

#[test]
fn enum_into_iter() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            #[new(into_iter)]
            x: Vec<u32>,
        },
    }

    let res = A::new_i(Some(5));
    assert_eq!(res, A::I { x: vec![5] });

    let res2 = A::new_i(None);
    assert_eq!(res2, A::I { x: vec![] });

    let res3 = A::new_i([1, 2, 3]);
    assert_eq!(res3, A::I { x: vec![1, 2, 3] });
}

#[test]
fn tuple_enum_into_iter() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(#[new(into_iter)] Vec<u32>),
    }

    let res = A::new_i(Some(5));
    assert_eq!(res, A::I(vec![5]));

    let res2 = A::new_i(None);
    assert_eq!(res2, A::I(vec![]));

    let res3 = A::new_i([1, 2, 3]);
    assert_eq!(res3, A::I(vec![1, 2, 3]));
}

#[test]
fn enum_alias_into_iter() {
    type X = Vec<u32>;

    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            #[new(into_iter = u32)]
            x: X,
        },
    }

    let res = A::new_i(Some(5));
    assert_eq!(res, A::I { x: vec![5] });
}

#[test]
fn tuple_enum_alias_into_iter() {
    type X = Vec<u32>;

    #[derive(Debug, PartialEq, New)]
    enum A {
        I(#[new(into_iter = u32)] X),
    }

    let res = A::new_i(Some(5));
    assert_eq!(res, A::I(vec![5]));
}

#[test]
fn enum_into_nested_iter() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            #[new(into_iter)]
            x: Vec<Vec<u32>>,
        },
    }

    let res = A::new_i(Some(vec![5]));
    assert_eq!(res, A::I { x: vec![vec![5]] });
}

#[test]
fn tuple_enum_into_nested_iter() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(#[new(into_iter)] Vec<Vec<u32>>),
    }

    let res = A::new_i(Some(vec![5]));
    assert_eq!(res, A::I(vec![vec![5]]));
}

#[test]
fn enum_alias_into_nested_iter() {
    type X = Vec<u32>;
    type Y = Vec<X>;

    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            #[new(into_iter = Vec<u32>)]
            y: Y,
        },
    }

    let res = A::new_i(Some(vec![5]));
    assert_eq!(res, A::I { y: vec![vec![5]] });
}

#[test]
fn tuple_enum_alias_into_nested_iter() {
    type X = Vec<u32>;
    type Y = Vec<X>;

    #[derive(Debug, PartialEq, New)]
    enum A {
        I(#[new(into_iter = Vec<u32>)] Y),
    }

    let res = A::new_i(Some(vec![5]));
    assert_eq!(res, A::I(vec![vec![5]]));
}

#[test]
fn enum_with_single_generic() {
    #[derive(Debug, PartialEq, New)]
    enum A<T> {
        I { x: T },
    }

    let res = A::new_i(1);
    assert_eq!(res, A::I { x: 1 });
}

#[test]
fn tuple_enum_with_single_generic() {
    #[derive(Debug, PartialEq, New)]
    enum A<T> {
        I(T),
    }

    let res = A::new_i(1);
    assert_eq!(res, A::I(1));
}

#[test]
fn enum_with_single_generic_and_another_field() {
    #[derive(Debug, PartialEq, New)]
    enum A<T> {
        I { x: T, y: u32 },
    }

    let res = A::new_i(1, 2);
    assert_eq!(res, A::I { x: 1, y: 2 });
}

#[test]
fn tuple_enum_with_single_generic_and_another_field() {
    #[derive(Debug, PartialEq, New)]
    enum A<T> {
        I(T, u32),
    }

    let res = A::new_i(1, 2);
    assert_eq!(res, A::I(1, 2));
}

#[test]
fn enum_with_multiple_generics() {
    #[derive(Debug, PartialEq, New)]
    enum A<X, Y> {
        I { x: X, y: Y },
    }

    let res = A::new_i(1u32, 2u64);
    assert_eq!(res, A::I { x: 1, y: 2 });
}

#[test]
fn tuple_enum_with_multiple_generics() {
    #[derive(Debug, PartialEq, New)]
    enum A<X, Y> {
        I(X, Y),
    }

    let res = A::new_i(1u32, 2u64);
    assert_eq!(res, A::I(1, 2));
}

#[test]
fn enum_with_nested_generics() {
    #[derive(Debug, PartialEq, New)]
    enum X<Y, Z> {
        J { y: Y, z: Z },
    }

    #[derive(Debug, PartialEq, New)]
    enum A<Y, Z> {
        I { x: X<Y, Z> },
    }

    let res = A::new_i(X::new_j(1, "z"));
    assert_eq!(
        res,
        A::I {
            x: X::J { y: 1, z: "z" }
        }
    );
}

#[test]
fn tuple_enum_with_nested_generics() {
    #[derive(Debug, PartialEq, New)]
    enum X<Y, Z> {
        J(Y, Z),
    }

    #[derive(Debug, PartialEq, New)]
    enum A<Y, Z> {
        I(X<Y, Z>),
    }

    let res = A::new_i(X::new_j(1, "z"));
    assert_eq!(res, A::I(X::J(1, "z")));
}

#[test]
fn enum_with_default_generics() {
    #[derive(Debug, PartialEq, New)]
    enum A<X, Y: Default> {
        I {
            x: X,
            #[new(default)]
            y: Y,
        },
    }

    let res = A::<u32, u64>::new_i(1u32);
    assert_eq!(res, A::I { x: 1, y: 0 });
}

#[test]
fn tuple_enum_with_default_generics() {
    #[derive(Debug, PartialEq, New)]
    enum A<X, Y: Default> {
        I(X, #[new(default)] Y),
    }

    let res = A::<u32, u64>::new_i(1u32);
    assert_eq!(res, A::I(1, 0));
}

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
fn enum_private_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(pub = false)]
    enum A<'a, T> {
        I { x: &'a T },
    }

    let x = 1u64;
    let res = A::new_i(&x);
    assert_eq!(res, A::I { x: &x });
}

#[test]
fn tuple_enum_private_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(pub = false)]
    enum A<'a, T> {
        I(&'a T),
    }

    let x = 1u64;
    let res = A::new_i(&x);
    assert_eq!(res, A::I(&x));
}

#[test]
fn enum_rename_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(rename = "create")]
    enum A<'a, T> {
        I { x: &'a T },
    }

    let x = 1u64;
    let res = A::create_i(&x);
    assert_eq!(res, A::I { x: &x });
}

#[test]
fn tuple_enum_rename_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(rename = "create")]
    enum A<'a, T> {
        I(&'a T),
    }

    let x = 1u64;
    let res = A::create_i(&x);
    assert_eq!(res, A::I(&x));
}

#[test]
fn enum_constructor_name() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        HTTPStatus { x: u32 },
        XMLHttpRequest { y: u64 },
        SimpleURLParser { z: u8 },
        XYZ { w: u16 },
    }

    let res = A::new_http_status(1);
    assert_eq!(res, A::HTTPStatus { x: 1 });

    let res2 = A::new_xml_http_request(2);
    assert_eq!(res2, A::XMLHttpRequest { y: 2 });

    let res3 = A::new_simple_url_parser(3);
    assert_eq!(res3, A::SimpleURLParser { z: 3 });

    let res4 = A::new_xyz(4);
    assert_eq!(res4, A::XYZ { w: 4 });
}

#[test]
fn tuple_enum_constructor_name() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        HTTPStatus(u32),
        XMLHttpRequest(u64),
        SimpleURLParser(u8),
        XYZ(u16),
    }

    let res = A::new_http_status(1);
    assert_eq!(res, A::HTTPStatus(1));

    let res2 = A::new_xml_http_request(2);
    assert_eq!(res2, A::XMLHttpRequest(2));

    let res3 = A::new_simple_url_parser(3);
    assert_eq!(res3, A::SimpleURLParser(3));

    let res4 = A::new_xyz(4);
    assert_eq!(res4, A::XYZ(4));
}

#[test]
fn enum_constructor_rename_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(rename = "create")]
    enum A {
        HTTPStatus { x: u32 },
        XMLHttpRequest { y: u64 },
        SimpleURLParser { z: u8 },
        XYZ { w: u16 },
    }

    let res = A::create_http_status(1);
    assert_eq!(res, A::HTTPStatus { x: 1 });

    let res2 = A::create_xml_http_request(2);
    assert_eq!(res2, A::XMLHttpRequest { y: 2 });

    let res3 = A::create_simple_url_parser(3);
    assert_eq!(res3, A::SimpleURLParser { z: 3 });

    let res4 = A::create_xyz(4);
    assert_eq!(res4, A::XYZ { w: 4 });
}

#[test]
fn tuple_enum_constructor_rename_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(rename = "create")]
    enum A {
        HTTPStatus(u32),
        XMLHttpRequest(u64),
        SimpleURLParser(u8),
        XYZ(u16),
    }

    let res = A::create_http_status(1);
    assert_eq!(res, A::HTTPStatus(1));

    let res2 = A::create_xml_http_request(2);
    assert_eq!(res2, A::XMLHttpRequest(2));

    let res3 = A::create_simple_url_parser(3);
    assert_eq!(res3, A::SimpleURLParser(3));

    let res4 = A::create_xyz(4);
    assert_eq!(res4, A::XYZ(4));
}
