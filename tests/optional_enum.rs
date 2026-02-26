use inew::New;

#[test]
fn enum_optional() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            x: u32,
            #[new(optional)]
            y: u64,
        },
    }

    let res = A::new_i(2, None);
    assert_eq!(res, A::I { x: 2, y: 0 });

    let res2 = A::new_i(2, Some(6u64));
    assert_eq!(res2, A::I { x: 2, y: 6 });
}

#[test]
fn tuple_enum_optional() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(u32, #[new(optional)] u64),
    }

    let res = A::new_i(2, None);
    assert_eq!(res, A::I(2, 0));

    let res2 = A::new_i(2, Some(6u64));
    assert_eq!(res2, A::I(2, 6));
}

#[test]
fn enum_optional_explicit_default() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            x: u32,
            #[new(optional)]
            y: u64,
        },
    }

    let res = A::new_i(2, None);
    assert_eq!(res, A::I { x: 2, y: 0 });

    let res2 = A::new_i(2, Some(6u64));
    assert_eq!(res2, A::I { x: 2, y: 6 });
}

#[test]
fn tuple_enum_optional_explicit_default() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(u32, #[new(optional)] u64),
    }

    let res = A::new_i(2, None);
    assert_eq!(res, A::I(2, 0));

    let res2 = A::new_i(2, Some(6u64));
    assert_eq!(res2, A::I(2, 6));
}

#[test]
fn enum_optional_expression() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            x: u32,
            #[new(default = 1 + 2, optional)]
            y: u64,
        },
    }

    let res = A::new_i(2, None);
    assert_eq!(res, A::I { x: 2, y: 3 });

    let res2 = A::new_i(2, Some(6u64));
    assert_eq!(res2, A::I { x: 2, y: 6 });
}

#[test]
fn const_enum_optional_expression() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I {
            x: u32,
            #[new(default = 1 + 2, optional)]
            y: u64,
        },
    }

    const RES: A = A::new_i(2, None);
    assert_eq!(RES, A::I { x: 2, y: 3 });

    const RES2: A = A::new_i(2, Some(6u64));
    assert_eq!(RES2, A::I { x: 2, y: 6 });
}

#[test]
fn tuple_enum_optional_expression() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(u32, #[new(default = 1 + 2, optional)] u64),
    }

    let res = A::new_i(2, None);
    assert_eq!(res, A::I(2, 3));

    let res2 = A::new_i(2, Some(6u64));
    assert_eq!(res2, A::I(2, 6));
}

#[test]
fn const_tuple_enum_optional_expression() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I(u32, #[new(default = 1 + 2, optional)] u64),
    }

    const RES: A = A::new_i(2, None);
    assert_eq!(RES, A::I(2, 3));

    const RES2: A = A::new_i(2, Some(6u64));
    assert_eq!(RES2, A::I(2, 6));
}

#[test]
fn enum_optional_custom_macro() {
    macro_rules! custom_macro {
        () => {
            7
        };
    }

    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            x: u32,
            #[new(default = custom_macro!(), optional)]
            y: u64,
        },
    }

    let res = A::new_i(2, None);
    assert_eq!(res, A::I { x: 2, y: 7 });

    let res2 = A::new_i(2, Some(6u64));
    assert_eq!(res2, A::I { x: 2, y: 6 });
}

#[test]
fn const_enum_optional_custom_macro() {
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
            #[new(default = custom_macro!(), optional)]
            y: u64,
        },
    }

    const RES: A = A::new_i(2, None);
    assert_eq!(RES, A::I { x: 2, y: 7 });

    const RES2: A = A::new_i(2, Some(6u64));
    assert_eq!(RES2, A::I { x: 2, y: 6 });
}

#[test]
fn tuple_enum_optional_custom_macro() {
    macro_rules! custom_macro {
        () => {
            7
        };
    }

    #[derive(Debug, PartialEq, New)]
    enum A {
        I(u32, #[new(default = custom_macro!(), optional)] u64),
    }

    let res = A::new_i(2, None);
    assert_eq!(res, A::I(2, 7));

    let res2 = A::new_i(2, Some(6u64));
    assert_eq!(res2, A::I(2, 6));
}

#[test]
fn const_tuple_enum_optional_custom_macro() {
    macro_rules! custom_macro {
        () => {
            7
        };
    }

    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I(u32, #[new(default = custom_macro!(), optional)] u64),
    }

    const RES: A = A::new_i(2, None);
    assert_eq!(RES, A::I(2, 7));

    const RES2: A = A::new_i(2, Some(6u64));
    assert_eq!(RES2, A::I(2, 6));
}

#[test]
fn enum_optional_allocation_macro() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            x: u32,
            #[new(default = vec![ 1u32 ], optional)]
            y: Vec<u32>,
        },
    }

    let res = A::new_i(2, None);
    assert_eq!(res, A::I { x: 2, y: vec![1] });

    let res2 = A::new_i(2, Some(vec![6u32]));
    assert_eq!(res2, A::I { x: 2, y: vec![6] });
}

#[test]
fn tuple_enum_optional_allocation_macro() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(u32, #[new(default = vec![ 1u32 ], optional)] Vec<u32>),
    }

    let res = A::new_i(2, None);
    assert_eq!(res, A::I(2, vec![1]));

    let res2 = A::new_i(2, Some(vec![6u32]));
    assert_eq!(res2, A::I(2, vec![6]));
}

#[test]
fn enum_optional_function() {
    fn custom_default() -> u64 {
        3
    }

    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            x: u32,
            #[new(default = custom_default(), optional)]
            y: u64,
        },
    }

    let res = A::new_i(2, None);
    assert_eq!(res, A::I { x: 2, y: 3 });

    let res2 = A::new_i(2, Some(6u64));
    assert_eq!(res2, A::I { x: 2, y: 6 });
}

#[test]
fn const_enum_optional_function() {
    const fn custom_default() -> u64 {
        3
    }

    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I {
            x: u32,
            #[new(default = custom_default(), optional)]
            y: u64,
        },
    }

    const RES: A = A::new_i(2, None);
    assert_eq!(RES, A::I { x: 2, y: 3 });

    const RES2: A = A::new_i(2, Some(6u64));
    assert_eq!(RES2, A::I { x: 2, y: 6 });
}

#[test]
fn tuple_enum_optional_function() {
    fn custom_default() -> u64 {
        3
    }

    #[derive(Debug, PartialEq, New)]
    enum A {
        I(u32, #[new(default = custom_default(), optional)] u64),
    }

    let res = A::new_i(2, None);
    assert_eq!(res, A::I(2, 3));

    let res2 = A::new_i(2, Some(6u64));
    assert_eq!(res2, A::I(2, 6));
}

#[test]
fn const_tuple_enum_optional_function() {
    const fn custom_default() -> u64 {
        3
    }

    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I(u32, #[new(default = custom_default(), optional)] u64),
    }

    const RES: A = A::new_i(2, None);
    assert_eq!(RES, A::I(2, 3));

    const RES2: A = A::new_i(2, Some(6u64));
    assert_eq!(RES2, A::I(2, 6));
}

#[test]
fn enum_with_nested_optional_function() {
    mod nested {
        pub fn custom_default() -> u64 {
            3
        }
    }

    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            x: u32,
            #[new(default = nested::custom_default(), optional)]
            y: u64,
        },
    }

    let res = A::new_i(2, None);
    assert_eq!(res, A::I { x: 2, y: 3 });

    let res2 = A::new_i(2, Some(6u64));
    assert_eq!(res2, A::I { x: 2, y: 6 });
}

#[test]
fn const_enum_with_nested_optional_function() {
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
            #[new(default = nested::custom_default(), optional)]
            y: u64,
        },
    }

    const RES: A = A::new_i(2, None);
    assert_eq!(RES, A::I { x: 2, y: 3 });

    const RES2: A = A::new_i(2, Some(6u64));
    assert_eq!(RES2, A::I { x: 2, y: 6 });
}

#[test]
fn tuple_enum_with_nested_optional_function() {
    mod nested {
        pub fn custom_default() -> u64 {
            3
        }
    }

    #[derive(Debug, PartialEq, New)]
    enum A {
        I(
            u32,
            #[new(default = nested::custom_default(), optional)] u64,
        ),
    }

    let res = A::new_i(2, None);
    assert_eq!(res, A::I(2, 3));

    let res2 = A::new_i(2, Some(6u64));
    assert_eq!(res2, A::I(2, 6));
}

#[test]
fn const_tuple_enum_with_nested_default_function() {
    mod nested {
        pub const fn custom_default() -> u64 {
            3
        }
    }

    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        I(
            u32,
            #[new(default = nested::custom_default(), optional)] u64,
        ),
    }

    const RES: A = A::new_i(2, None);
    assert_eq!(RES, A::I(2, 3));

    const RES2: A = A::new_i(2, Some(6u64));
    assert_eq!(RES2, A::I(2, 6));
}

#[test]
fn enum_with_all_optional_defaults() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            #[new(optional)]
            x: u32,
            #[new(optional)]
            y: u64,
        },
    }

    let res = A::new_i(None, None);
    assert_eq!(res, A::I { x: 0, y: 0 });

    let res2 = A::new_i(Some(6u32), Some(8u64));
    assert_eq!(res2, A::I { x: 6, y: 8 });
}

#[test]
fn tuple_enum_with_all_optional_defaults() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(#[new(optional)] u32, #[new(optional)] u64),
    }

    let res = A::new_i(None, None);
    assert_eq!(res, A::I(0, 0));

    let res2 = A::new_i(Some(6u32), Some(8u64));
    assert_eq!(res2, A::I(6, 8));
}

#[test]
fn enum_with_mixed_all_optional_defaults() {
    fn custom_default() -> u8 {
        5
    }

    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            #[new(optional)]
            x: u32,
            #[new(default = 1 + 2, optional)]
            y: u64,
            #[new(default = custom_default(), optional)]
            z: u8,
        },
    }

    let res = A::new_i(None, None, None);
    assert_eq!(res, A::I { x: 0, y: 3, z: 5 });

    let res2 = A::new_i(Some(6u32), Some(8u64), Some(10u8));
    assert_eq!(res2, A::I { x: 6, y: 8, z: 10 });
}

#[test]
fn tuple_enum_with_mixed_all_optional_defaults() {
    fn custom_default() -> u8 {
        5
    }

    #[derive(Debug, PartialEq, New)]
    enum A {
        I(
            #[new(optional)] u32,
            #[new(default = 1 + 2, optional)] u64,
            #[new(default = custom_default(), optional)] u8,
        ),
    }

    let res = A::new_i(None, None, None);
    assert_eq!(res, A::I(0, 3, 5));

    let res2 = A::new_i(Some(6u32), Some(8u64), Some(10u8));
    assert_eq!(res2, A::I(6, 8, 10));
}
