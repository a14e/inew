use inew::New;

#[test]
fn enum_public_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(pub)]
    enum A {
        I { x: u32 },
    }

    #[derive(Debug, PartialEq, New)]
    #[new(pub = true)]
    enum B {
        J { x: u32 },
    }

    let res = A::new_i(1);
    assert_eq!(res, A::I { x: 1 });

    let res2 = B::new_j(2);
    assert_eq!(res2, B::J { x: 2 });
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
fn tuple_enum_public_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(pub)]
    enum A {
        I(u32),
    }

    #[derive(Debug, PartialEq, New)]
    #[new(pub = true)]
    enum B {
        J(u32),
    }

    let res = A::new_i(1);
    assert_eq!(res, A::I(1));

    let res2 = B::new_j(2);
    assert_eq!(res2, B::J(2));
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
fn enum_explicit_private_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(pub = false)]
    enum A {
        I { x: u32 },
    }

    let res = A::new_i(1);
    assert_eq!(res, A::I { x: 1 });
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
fn tuple_enum_explicit_private_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(pub = false)]
    enum A {
        I(u32),
    }

    let res = A::new_i(1);
    assert_eq!(res, A::I(1));
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
fn enum_custom_visibility() {
    #[derive(Debug, PartialEq, New)]
    #[new(pub(crate))]
    enum A {
        I { x: u32 },
    }

    mod nested {
        use super::*;

        #[derive(Debug, PartialEq, New)]
        #[new(pub(super))]
        pub enum B {
            I { x: u32 },
        }
    }

    #[derive(Debug, PartialEq, New)]
    #[new(pub(self))]
    enum C {
        I { x: u32 },
    }

    let res = A::new_i(1);
    assert_eq!(res, A::I { x: 1 });

    let res2 = nested::B::new_i(2);
    assert_eq!(res2, nested::B::I { x: 2 });

    let res3 = C::new_i(3);
    assert_eq!(res3, C::I { x: 3 });
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
fn tuple_enum_custom_visibility() {
    #[derive(Debug, PartialEq, New)]
    #[new(pub(crate))]
    enum A {
        I(u32),
    }

    mod nested {
        use super::*;

        #[derive(Debug, PartialEq, New)]
        #[new(pub(super))]
        pub enum B {
            I(u32),
        }
    }

    #[derive(Debug, PartialEq, New)]
    #[new(pub(self))]
    enum C {
        I(u32),
    }

    let res = A::new_i(1);
    assert_eq!(res, A::I(1));

    let res2 = nested::B::new_i(2);
    assert_eq!(res2, nested::B::I(2));

    let res3 = C::new_i(3);
    assert_eq!(res3, C::I(3));
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
