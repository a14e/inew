#![cfg_attr(not(feature = "std"), no_std)]

use inew::New;

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
fn const_struct_type_alias() {
    type X = u32;

    #[derive(New)]
    #[new(const)]
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
    struct A(X);

    let res = A::new(2);
    assert_eq!(res.0, 2);
}

#[test]
fn const_tuple_struct_type_alias() {
    type X = u32;

    #[derive(New)]
    #[new(const)]
    struct A(X);

    const RES: A = A::new(2);
    assert_eq!(RES.0, 2);
}

#[test]
fn struct_use_alias() {
    mod nested {
        #[derive(Debug, PartialEq)]
        pub struct Struct(pub u32);
    }

    use nested::Struct as S;

    #[derive(New)]
    struct A {
        x: S,
    }

    let res = A::new(S(1));
    assert_eq!(res.x.0, 1);
}

#[test]
fn const_struct_use_alias() {
    mod nested {
        #[derive(Debug, PartialEq)]
        pub struct Struct(pub u32);
    }

    use nested::Struct as S;

    #[derive(New)]
    #[new(const)]
    struct A {
        x: S,
    }

    const RES: A = A::new(S(1));
    assert_eq!(RES.x.0, 1);
}

#[test]
fn tuple_struct_use_alias() {
    mod nested {
        #[derive(Debug, PartialEq)]
        pub struct Struct(pub u32);
    }

    use nested::Struct as S;

    #[derive(New)]
    struct A(S);

    let res = A::new(S(1));
    assert_eq!(res.0 .0, 1);
}

#[test]
fn const_tuple_struct_use_alias() {
    mod nested {
        #[derive(Debug, PartialEq)]
        pub struct Struct(pub u32);
    }

    use nested::Struct as S;

    #[derive(New)]
    #[new(const)]
    struct A(S);

    const RES: A = A::new(S(1));
    assert_eq!(RES.0 .0, 1);
}
