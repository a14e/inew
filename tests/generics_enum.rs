use inew::New;

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
fn tuple_enum_with_single_generic() {
    #[derive(Debug, PartialEq, New)]
    enum A<T> {
        I(T),
    }

    let res = A::new_i(1);
    assert_eq!(res, A::I(1));
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
fn enum_with_single_generic_and_another_field() {
    #[derive(Debug, PartialEq, New)]
    enum A<T> {
        I { x: T, y: u32 },
    }

    let res = A::new_i(1, 2);
    assert_eq!(res, A::I { x: 1, y: 2 });
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
fn tuple_enum_with_single_generic_and_another_field() {
    #[derive(Debug, PartialEq, New)]
    enum A<T> {
        I(T, u32),
    }

    let res = A::new_i(1, 2);
    assert_eq!(res, A::I(1, 2));
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
fn enum_with_multiple_generics() {
    #[derive(Debug, PartialEq, New)]
    enum A<X, Y> {
        I { x: X, y: Y },
    }

    let res = A::new_i(1u32, 2u64);
    assert_eq!(res, A::I { x: 1, y: 2 });
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
fn tuple_enum_with_multiple_generics() {
    #[derive(Debug, PartialEq, New)]
    enum A<X, Y> {
        I(X, Y),
    }

    let res = A::new_i(1u32, 2u64);
    assert_eq!(res, A::I(1, 2));
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
