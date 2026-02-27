#![cfg_attr(not(feature = "std"), no_std)]

use inew::New;

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
fn tuple_enum_single_field() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(u32),
    }

    let res = A::new_i(1);
    assert_eq!(res, A::I(1));
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
fn enum_explicit_const() {
    #[derive(Debug, PartialEq, New)]
    #[new(const = false)]
    enum A {
        I { x: u32 },
    }

    let res = A::new_i(1);
    assert_eq!(res, A::I { x: 1 });
}

#[test]
fn const_enum_explicit_const() {
    #[derive(Debug, PartialEq, New)]
    #[new(const = true)]
    enum A {
        I { x: u32 },
    }

    const RES: A = A::new_i(1);
    assert_eq!(RES, A::I { x: 1 });
}

#[test]
fn tuple_enum_explicit_const() {
    #[derive(Debug, PartialEq, New)]
    #[new(const = false)]
    enum A {
        I(u32),
    }

    let res = A::new_i(1);
    assert_eq!(res, A::I(1));
}

#[test]
fn const_tuple_enum_explicit_const() {
    #[derive(Debug, PartialEq, New)]
    #[new(const = true)]
    enum A {
        I(u32),
    }

    const RES: A = A::new_i(1);
    assert_eq!(RES, A::I(1));
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
fn tuple_enum_multiple_fields() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(u32, u64),
    }

    let res = A::new_i(2, 3);
    assert_eq!(res, A::I(2, 3));
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
fn enum_mixed_variants() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I,
        J { x: u32 },
        K(u64, u64),
        L { x: u8, y: u8 },
        M(u16, u16, u16, bool),
    }

    let res = A::new_i();
    assert_eq!(res, A::I);

    let res2 = A::new_j(1);
    assert_eq!(res2, A::J { x: 1 });

    let res3 = A::new_k(1, 2);
    assert_eq!(res3, A::K(1, 2));

    let res4 = A::new_l(1, 2);
    assert_eq!(res4, A::L { x: 1, y: 2 });

    let res5 = A::new_m(1, 2, 3, true);
    assert_eq!(res5, A::M(1, 2, 3, true));
}

#[test]
fn const_enum_mixed_variants() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I,
        J { x: u32 },
        K(u64, u64),
        L { x: u8, y: u8 },
        M(u16, u16, u16, bool),
    }

    const RES: A = A::new_i();
    assert_eq!(RES, A::I);

    const RES2: A = A::new_j(1);
    assert_eq!(RES2, A::J { x: 1 });

    const RES3: A = A::new_k(1, 2);
    assert_eq!(RES3, A::K(1, 2));

    const RES4: A = A::new_l(1, 2);
    assert_eq!(RES4, A::L { x: 1, y: 2 });

    const RES5: A = A::new_m(1, 2, 3, true);
    assert_eq!(RES5, A::M(1, 2, 3, true));
}

#[test]
fn tuple_enum_mixed_variants() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I,
        J(u32),
        K(u64, u64),
        L(u8, u8),
        M(u16, u16, u16, bool),
    }

    let res = A::new_i();
    assert_eq!(res, A::I);

    let res2 = A::new_j(1);
    assert_eq!(res2, A::J(1));

    let res3 = A::new_k(1, 2);
    assert_eq!(res3, A::K(1, 2));

    let res4 = A::new_l(1, 2);
    assert_eq!(res4, A::L(1, 2));

    let res5 = A::new_m(1, 2, 3, true);
    assert_eq!(res5, A::M(1, 2, 3, true));
}

#[test]
fn const_tuple_enum_mixed_variants() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I,
        J(u32),
        K(u64, u64),
        L(u8, u8),
        M(u16, u16, u16, bool),
    }

    const RES: A = A::new_i();
    assert_eq!(RES, A::I);

    const RES2: A = A::new_j(1);
    assert_eq!(RES2, A::J(1));

    const RES3: A = A::new_k(1, 2);
    assert_eq!(RES3, A::K(1, 2));

    const RES4: A = A::new_l(1, 2);
    assert_eq!(RES4, A::L(1, 2));

    const RES5: A = A::new_m(1, 2, 3, true);
    assert_eq!(RES5, A::M(1, 2, 3, true));
}
