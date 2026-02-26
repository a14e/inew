use inew::New;
use std::marker::PhantomData;

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
fn tuple_enum_unit_auto_default() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(()),
    }

    let res = A::new_i();
    assert_eq!(res, A::I(()));
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
fn enum_phantom_data_auto_default() {
    #[derive(Debug, PartialEq, New)]
    enum A<T> {
        I { x: PhantomData<T> },
    }

    let res = A::<u32>::new_i();
    assert_eq!(res, A::I { x: PhantomData });
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
fn tuple_enum_phantom_data_auto_default() {
    #[derive(Debug, PartialEq, New)]
    enum A<T> {
        I(PhantomData<T>),
    }

    let res = A::<u32>::new_i();
    assert_eq!(res, A::I(PhantomData));
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
fn tuple_enum_with_default_expression() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(u32, #[new(default = 1 + 2)] u64),
    }

    let res = A::new_i(2);
    assert_eq!(res, A::I(2, 3));
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
fn const_enum_with_default_function() {
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
fn const_tuple_enum_with_default_function() {
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
fn const_enum_with_nested_default_function() {
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
fn const_tuple_enum_with_nested_default_function() {
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
