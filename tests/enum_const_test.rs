use inew::New;
use std::marker::PhantomData;

#[test]
fn const_unit_like_enum_with_braces() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I {},
    }

    const RES: A = A::new_i();
    assert_eq!(RES, A::I {});
}

#[test]
fn const_unit_like_enum_without_braces() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I,
    }

    const RES: A = A::new_i();
    assert_eq!(RES, A::I);
}

#[test]
fn const_unit_like_enum_with_parentheses() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I(),
    }

    const RES: A = A::new_i();
    assert_eq!(RES, A::I());
}

#[test]
fn const_enum_single_field() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I { x: u32 },
    }

    const RES: A = A::new_i(1);
    assert_eq!(RES, A::I { x: 1 });
}

#[test]
fn const_tuple_enum_single_field() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I(u32),
    }

    const RES: A = A::new_i(1);
    assert_eq!(RES, A::I(1));
}

#[test]
fn const_enum_explicit_syntax() {
    #[derive(Debug, PartialEq, New)]
    #[new(const = true)]
    enum A {
        I { x: u32 },
    }

    const RES: A = A::new_i(1);
    assert_eq!(RES, A::I { x: 1 });
}

#[test]
fn const_tuple_enum_explicit_syntax() {
    #[derive(Debug, PartialEq, New)]
    #[new(const = true)]
    enum A {
        I(u32),
    }

    const RES: A = A::new_i(1);
    assert_eq!(RES, A::I(1));
}

#[test]
fn const_enum_multiple_fields() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I { x: u32, y: u64 },
    }

    const RES: A = A::new_i(2, 3);
    assert_eq!(RES, A::I { x: 2, y: 3 });
}

#[test]
fn const_tuple_enum_multiple_fields() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I(u32, u64),
    }

    const RES: A = A::new_i(2, 3);
    assert_eq!(RES, A::I(2, 3));
}

#[test]
fn const_enum_type_alias() {
    type X = u32;

    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I { x: X },
    }

    const RES: A = A::new_i(2);
    assert_eq!(RES, A::I { x: 2 });
}

#[test]
fn const_tuple_enum_type_alias() {
    type X = u32;

    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I(X),
    }

    const RES: A = A::new_i(2);
    assert_eq!(RES, A::I(2));
}

#[test]
fn enum_use_alias() {
    mod nested {
        #[derive(Debug, PartialEq)]
        pub struct Struct(pub u32);
    }

    use nested::Struct as S;

    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I { x: S },
    }

    const RES: A = A::new_i(S(1));
    assert_eq!(RES, A::I { x: S(1) });
}

#[test]
fn tuple_enum_use_alias() {
    mod nested {
        #[derive(Debug, PartialEq)]
        pub struct Struct(pub u32);
    }

    use nested::Struct as S;

    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I(S),
    }

    const RES: A = A::new_i(S(1));
    assert_eq!(RES, A::I(S(1)));
}

#[test]
fn const_enum_mixed_variants() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum MyEnum {
        I,
        J { x: u32 },
        K(u64, u64),
        L { x: u8, y: u8 },
        M(u16, u16, u16, bool),
    }

    const RES: MyEnum = MyEnum::new_i();
    assert_eq!(RES, MyEnum::I);

    const RES2: MyEnum = MyEnum::new_j(1);
    assert_eq!(RES2, MyEnum::J { x: 1 });

    const RES3: MyEnum = MyEnum::new_k(1, 2);
    assert_eq!(RES3, MyEnum::K(1, 2));

    const RES4: MyEnum = MyEnum::new_l(1, 2);
    assert_eq!(RES4, MyEnum::L { x: 1, y: 2 });

    const RES5: MyEnum = MyEnum::new_m(1, 2, 3, true);
    assert_eq!(RES5, MyEnum::M(1, 2, 3, true));
}

#[test]
fn const_enum_unit_auto_default() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I { x: () },
    }

    const RES: A = A::new_i();
    assert_eq!(RES, A::I { x: () });
}

#[test]
fn const_tuple_enum_unit_auto_default() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I(()),
    }

    const RES: A = A::new_i();
    assert_eq!(RES, A::I(()));
}

#[test]
fn const_enum_phantom_data_auto_default() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A<T> {
        I { x: PhantomData<T> },
    }

    const RES: A<u32> = A::new_i();
    assert_eq!(RES, A::I { x: PhantomData });
}

#[test]
fn const_tuple_enum_phantom_data_auto_default() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A<T> {
        I(PhantomData<T>),
    }

    const RES: A<u32> = A::new_i();
    assert_eq!(RES, A::I(PhantomData));
}

#[test]
fn const_enum_with_default_expression() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I {
            x: u32,
            #[new(default = 1 + 2)]
            y: u64,
        },
    }

    const RES: A = A::new_i(2);
    assert_eq!(RES, A::I { x: 2, y: 3 });
}

#[test]
fn const_tuple_enum_with_default_expression() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I(u32, #[new(default = 1 + 2)] u64),
    }

    const RES: A = A::new_i(2);
    assert_eq!(RES, A::I(2, 3));
}

#[test]
fn const_enum_with_default_custom_macro() {
    macro_rules! custom_macro {
        () => {
            7
        };
    }

    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I {
            x: u32,
            #[new(default = custom_macro!())]
            y: u64,
        },
    }

    const RES: A = A::new_i(2);
    assert_eq!(RES, A::I { x: 2, y: 7 });
}

#[test]
fn const_tuple_enum_with_default_custom_macro() {
    macro_rules! custom_macro {
        () => {
            7
        };
    }

    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I(u32, #[new(default = custom_macro!())] u64),
    }

    const RES: A = A::new_i(2);
    assert_eq!(RES, A::I(2, 7));
}

#[test]
fn const_enum_with_default_const_function() {
    const fn custom_default() -> u64 {
        3
    }

    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I {
            x: u32,
            #[new(default = custom_default())]
            y: u64,
        },
    }

    const RES: A = A::new_i(2);
    assert_eq!(RES, A::I { x: 2, y: 3 });
}

#[test]
fn const_tuple_enum_with_default_const_function() {
    const fn custom_default() -> u64 {
        3
    }

    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I(u32, #[new(default = custom_default())] u64),
    }

    const RES: A = A::new_i(2);
    assert_eq!(RES, A::I(2, 3));
}

#[test]
fn const_enum_with_nested_default_const_function() {
    mod nested {
        pub const fn custom_default() -> u64 {
            3
        }
    }

    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I {
            x: u32,
            #[new(default = nested::custom_default())]
            y: u64,
        },
    }

    const RES: A = A::new_i(2);
    assert_eq!(RES, A::I { x: 2, y: 3 });
}

#[test]
fn const_tuple_enum_with_nested_default_const_function() {
    mod nested {
        pub const fn custom_default() -> u64 {
            3
        }
    }

    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I(u32, #[new(default = nested::custom_default())] u64),
    }

    const RES: A = A::new_i(2);
    assert_eq!(RES, A::I(2, 3));
}

#[test]
fn const_enum_with_single_generic() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A<T> {
        I { x: T },
    }

    const RES: A<u32> = A::new_i(1);
    assert_eq!(RES, A::I { x: 1 });
}

#[test]
fn const_tuple_enum_with_single_generic() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A<T> {
        I(T),
    }

    const RES: A<u32> = A::new_i(1);
    assert_eq!(RES, A::I(1));
}

#[test]
fn const_enum_with_single_generic_and_another_field() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A<T> {
        I { x: T, y: u32 },
    }

    const RES: A<u32> = A::new_i(1, 2);
    assert_eq!(RES, A::I { x: 1, y: 2 });
}

#[test]
fn const_tuple_enum_with_single_generic_and_another_field() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A<T> {
        I(T, u32),
    }

    const RES: A<u32> = A::new_i(1, 2);
    assert_eq!(RES, A::I(1, 2));
}

#[test]
fn const_enum_with_multiple_generics() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A<X, Y> {
        I { x: X, y: Y },
    }

    const RES: A<u32, u64> = A::new_i(1u32, 2u64);
    assert_eq!(RES, A::I { x: 1, y: 2 });
}

#[test]
fn const_tuple_enum_with_multiple_generics() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A<X, Y> {
        I(X, Y),
    }

    const RES: A<u32, u64> = A::new_i(1u32, 2u64);
    assert_eq!(RES, A::I(1, 2));
}

#[test]
fn const_enum_with_nested_generics() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum X<Y, Z> {
        J { y: Y, z: Z },
    }

    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A<Y, Z> {
        I { x: X<Y, Z> },
    }

    const RES: A<u32, &str> = A::new_i(X::new_j(1, "z"));
    assert_eq!(
        RES,
        A::I {
            x: X::J { y: 1, z: "z" }
        }
    );
}

#[test]
fn const_tuple_enum_with_nested_generics() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum X<Y, Z> {
        J(Y, Z),
    }

    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A<Y, Z> {
        I(X<Y, Z>),
    }

    const RES: A<u32, &str> = A::new_i(X::new_j(1, "z"));
    assert_eq!(RES, A::I(X::J(1, "z")));
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
fn const_enum_dyn_function() {
    #[derive(New)]
    #[new(const)]
    enum A<'a> {
        I { f: &'a dyn Fn(f32) -> String },
    }

    const F: fn(f32) -> String = |x: f32| x.to_string();
    const RES: A = A::new_i(&F);
    match RES {
        A::I { f } => assert_eq!((f)(3.14), "3.14"),
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
        A::I(f) => assert_eq!((f)(3.14), "3.14"),
    }
}

#[test]
fn const_enum_public_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(pub, const)]
    enum A {
        I { x: u32 },
    }

    #[derive(Debug, PartialEq, New)]
    #[new(pub = true, const)]
    enum B {
        J { x: u32 },
    }

    const RES: A = A::new_i(1);
    assert_eq!(RES, A::I { x: 1 });

    const RES2: B = B::new_j(2);
    assert_eq!(RES2, B::J { x: 2 });
}

#[test]
fn const_tuple_enum_public_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(pub, const)]
    enum A {
        I(u32),
    }

    #[derive(Debug, PartialEq, New)]
    #[new(pub = true, const)]
    enum B {
        J(u32),
    }

    const RES: A = A::new_i(1);
    assert_eq!(RES, A::I(1));

    const RES2: B = B::new_j(2);
    assert_eq!(RES2, B::J(2));
}

#[test]
fn const_enum_explicit_private_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(pub = false, const)]
    enum A {
        I { x: u32 },
    }

    const RES: A = A::new_i(1);
    assert_eq!(RES, A::I { x: 1 });
}

#[test]
fn const_tuple_enum_explicit_private_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(pub = false, const)]
    enum A {
        I(u32),
    }

    const RES: A = A::new_i(1);
    assert_eq!(RES, A::I(1));
}

#[test]
fn const_enum_custom_visibility() {
    #[derive(Debug, PartialEq, New)]
    #[new(pub(crate), const)]
    enum A {
        I { x: u32 },
    }

    mod nested {
        use super::*;

        #[derive(Debug, PartialEq, New)]
        #[new(pub(super), const)]
        pub enum B {
            I { x: u32 },
        }
    }

    #[derive(Debug, PartialEq, New)]
    #[new(pub(self), const)]
    enum C {
        I { x: u32 },
    }

    const RES: A = A::new_i(1);
    assert_eq!(RES, A::I { x: 1 });

    const RES2: nested::B = nested::B::new_i(2);
    assert_eq!(RES2, nested::B::I { x: 2 });

    const RES3: C = C::new_i(3);
    assert_eq!(RES3, C::I { x: 3 });
}

#[test]
fn const_tuple_enum_custom_visibility() {
    #[derive(Debug, PartialEq, New)]
    #[new(pub(crate), const)]
    enum A {
        I(u32),
    }

    mod nested {
        use super::*;

        #[derive(Debug, PartialEq, New)]
        #[new(pub(super), const)]
        pub enum B {
            I(u32),
        }
    }

    #[derive(Debug, PartialEq, New)]
    #[new(pub(self), const)]
    enum C {
        I(u32),
    }

    const RES: A = A::new_i(1);
    assert_eq!(RES, A::I(1));

    const RES2: nested::B = nested::B::new_i(2);
    assert_eq!(RES2, nested::B::I(2));

    const RES3: C = C::new_i(3);
    assert_eq!(RES3, C::I(3));
}

#[test]
fn const_enum_rename_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(rename = "create", const)]
    enum A {
        I { x: u32 },
    }

    const RES: A = A::create_i(1);
    assert_eq!(RES, A::I { x: 1 });
}

#[test]
fn const_tuple_enum_rename_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(rename = "create", const)]
    enum A {
        I(u32),
    }

    const RES: A = A::create_i(1);
    assert_eq!(RES, A::I(1));
}

#[test]
fn const_enum_constructor_name() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        HTTPStatus { x: u32 },
        XMLHttpRequest { y: u64 },
        SimpleURLParser { z: u8 },
        XYZ { w: u16 },
    }

    const RES: A = A::new_http_status(1);
    assert_eq!(RES, A::HTTPStatus { x: 1 });

    const RES2: A = A::new_xml_http_request(2);
    assert_eq!(RES2, A::XMLHttpRequest { y: 2 });

    const RES3: A = A::new_simple_url_parser(3);
    assert_eq!(RES3, A::SimpleURLParser { z: 3 });

    const RES4: A = A::new_xyz(4);
    assert_eq!(RES4, A::XYZ { w: 4 });
}

#[test]
fn const_tuple_enum_constructor_name() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        HTTPStatus(u32),
        XMLHttpRequest(u64),
        SimpleURLParser(u8),
        XYZ(u16),
    }

    const RES: A = A::new_http_status(1);
    assert_eq!(RES, A::HTTPStatus(1));

    const RES2: A = A::new_xml_http_request(2);
    assert_eq!(RES2, A::XMLHttpRequest(2));

    const RES3: A = A::new_simple_url_parser(3);
    assert_eq!(RES3, A::SimpleURLParser(3));

    const RES4: A = A::new_xyz(4);
    assert_eq!(RES4, A::XYZ(4));
}

#[test]
fn enum_constructor_rename_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(rename = "create", const)]
    enum A {
        HTTPStatus { x: u32 },
        XMLHttpRequest { y: u64 },
        SimpleURLParser { z: u8 },
        XYZ { w: u16 },
    }

    const RES: A = A::create_http_status(1);
    assert_eq!(RES, A::HTTPStatus { x: 1 });

    const RES2: A = A::create_xml_http_request(2);
    assert_eq!(RES2, A::XMLHttpRequest { y: 2 });

    const RES3: A = A::create_simple_url_parser(3);
    assert_eq!(RES3, A::SimpleURLParser { z: 3 });

    const RES4: A = A::create_xyz(4);
    assert_eq!(RES4, A::XYZ { w: 4 });
}

#[test]
fn tuple_enum_constructor_rename_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(rename = "create", const)]
    enum A {
        HTTPStatus(u32),
        XMLHttpRequest(u64),
        SimpleURLParser(u8),
        XYZ(u16),
    }

    const RES: A = A::create_http_status(1);
    assert_eq!(RES, A::HTTPStatus(1));

    const RES2: A = A::create_xml_http_request(2);
    assert_eq!(RES2, A::XMLHttpRequest(2));

    const RES3: A = A::create_simple_url_parser(3);
    assert_eq!(RES3, A::SimpleURLParser(3));

    const RES4: A = A::create_xyz(4);
    assert_eq!(RES4, A::XYZ(4));
}

#[test]
fn const_enum_no_prefix() {
    #[derive(Debug, PartialEq, New)]
    #[new(no_prefix, const)]
    enum A {
        I { x: u32 },
        J { y: u64 },
        K { z: u8 },
    }

    const RES: A = A::i(1);
    assert_eq!(RES, A::I { x: 1 });

    const RES2: A = A::j(2);
    assert_eq!(RES2, A::J { y: 2 });

    const RES3: A = A::k(3);
    assert_eq!(RES3, A::K { z: 3 });
}

#[test]
fn const_tuple_enum_no_prefix() {
    #[derive(Debug, PartialEq, New)]
    #[new(no_prefix, const)]
    enum A {
        I(u32),
        J(u64),
        K(u8),
    }

    const RES: A = A::i(1);
    assert_eq!(RES, A::I(1));

    const RES2: A = A::j(2);
    assert_eq!(RES2, A::J(2));

    const RES3: A = A::k(3);
    assert_eq!(RES3, A::K(3));
}
