use inew::New;

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
    enum A {
        I { x: S },
    }

    let res = A::new_i(S(1));
    assert_eq!(res, A::I { x: S(1) });
}

#[test]
fn const_enum_use_alias() {
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
    enum A {
        I(S),
    }

    let res = A::new_i(S(1));
    assert_eq!(res, A::I(S(1)));
}

#[test]
fn const_tuple_enum_use_alias() {
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
