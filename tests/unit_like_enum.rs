use inew::New;

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
fn unit_like_enum_without_braces() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I,
    }

    let res = A::new_i();
    assert_eq!(res, A::I);
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
fn unit_like_enum_with_parentheses() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(),
    }

    let res = A::new_i();
    assert_eq!(res, A::I());
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
