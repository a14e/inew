use inew::New;

#[test]
fn struct_optional() {
    #[derive(New)]
    struct A {
        x: u32,
        #[new(optional)]
        y: u64,
    }

    let res = A::new(2, None);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, 0);

    let res2 = A::new(2, Some(6u64));
    assert_eq!(res2.x, 2);
    assert_eq!(res2.y, 6);
}

#[test]
fn tuple_struct_optional() {
    #[derive(New)]
    struct A(u32, #[new(optional)] u64);

    let res = A::new(2, None);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 0);

    let res2 = A::new(2, Some(6u64));
    assert_eq!(res2.0, 2);
    assert_eq!(res2.1, 6);
}

#[test]
fn struct_optional_explicit_default() {
    #[derive(New)]
    struct A {
        x: u32,
        #[new(optional)]
        y: u64,
    }

    let res = A::new(2, None);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, 0);

    let res2 = A::new(2, Some(6u64));
    assert_eq!(res2.x, 2);
    assert_eq!(res2.y, 6);
}

#[test]
fn tuple_struct_optional_explicit_default() {
    #[derive(New)]
    struct A(u32, #[new(optional)] u64);

    let res = A::new(2, None);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 0);

    let res2 = A::new(2, Some(6u64));
    assert_eq!(res2.0, 2);
    assert_eq!(res2.1, 6);
}

#[test]
fn struct_optional_expression() {
    #[derive(New)]
    struct A {
        x: u32,
        #[new(default = 1 + 2, optional)]
        y: u64,
    }

    let res = A::new(2, None);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, 3);

    let res2 = A::new(2, Some(6u64));
    assert_eq!(res2.x, 2);
    assert_eq!(res2.y, 6);
}

#[test]
fn const_struct_optional_expression() {
    #[derive(New)]
    #[new(const)]
    struct A {
        x: u32,
        #[new(default = 1 + 2, optional)]
        y: u64,
    }

    const RES: A = A::new(2, None);
    assert_eq!(RES.x, 2);
    assert_eq!(RES.y, 3);

    const RES2: A = A::new(2, Some(6u64));
    assert_eq!(RES2.x, 2);
    assert_eq!(RES2.y, 6);
}

#[test]
fn tuple_struct_optional_expression() {
    #[derive(New)]
    struct A(u32, #[new(default = 1 + 2, optional)] u64);

    let res = A::new(2, None);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 3);

    let res2 = A::new(2, Some(6u64));
    assert_eq!(res2.0, 2);
    assert_eq!(res2.1, 6);
}

#[test]
fn const_tuple_struct_optional_expression() {
    #[derive(New)]
    #[new(const)]
    struct A(u32, #[new(default = 1 + 2, optional)] u64);

    const RES: A = A::new(2, None);
    assert_eq!(RES.0, 2);
    assert_eq!(RES.1, 3);

    const RES2: A = A::new(2, Some(6u64));
    assert_eq!(RES2.0, 2);
    assert_eq!(RES2.1, 6);
}

#[test]
fn struct_optional_custom_macro() {
    macro_rules! custom_macro {
        () => {
            7
        };
    }

    #[derive(New)]
    struct A {
        x: u32,
        #[new(default = custom_macro!(), optional)]
        y: u64,
    }

    let res = A::new(2, None);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, 7);

    let res2 = A::new(2, Some(6u64));
    assert_eq!(res2.x, 2);
    assert_eq!(res2.y, 6);
}

#[test]
fn const_struct_optional_custom_macro() {
    macro_rules! custom_macro {
        () => {
            7
        };
    }

    #[derive(New)]
    #[new(const)]
    struct A {
        x: u32,
        #[new(default = custom_macro!(), optional)]
        y: u64,
    }

    const RES: A = A::new(2, None);
    assert_eq!(RES.x, 2);
    assert_eq!(RES.y, 7);

    const RES2: A = A::new(2, Some(6u64));
    assert_eq!(RES2.x, 2);
    assert_eq!(RES2.y, 6);
}

#[test]
fn tuple_struct_optional_custom_macro() {
    macro_rules! custom_macro {
        () => {
            7
        };
    }

    #[derive(New)]
    struct A(u32, #[new(default = custom_macro!(), optional)] u64);

    let res = A::new(2, None);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 7);

    let res2 = A::new(2, Some(6u64));
    assert_eq!(res2.0, 2);
    assert_eq!(res2.1, 6);
}

#[test]
fn const_tuple_struct_optional_custom_macro() {
    macro_rules! custom_macro {
        () => {
            7
        };
    }

    #[derive(New)]
    #[new(const)]
    struct A(u32, #[new(default = custom_macro!(), optional)] u64);

    const RES: A = A::new(2, None);
    assert_eq!(RES.0, 2);
    assert_eq!(RES.1, 7);

    const RES2: A = A::new(2, Some(6u64));
    assert_eq!(RES2.0, 2);
    assert_eq!(RES2.1, 6);
}

#[test]
fn struct_optional_allocation_macro() {
    #[derive(New)]
    struct A {
        x: u32,
        #[new(default = vec![1u32], optional)]
        y: Vec<u32>,
    }

    let res = A::new(2, None);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, vec![1]);

    let res2 = A::new(2, Some(vec![6u32]));
    assert_eq!(res2.x, 2);
    assert_eq!(res2.y, vec![6]);
}

#[test]
fn tuple_struct_optional_allocation_macro() {
    #[derive(New)]
    struct A(u32, #[new(default = vec![ 1u32 ], optional)] Vec<u32>);

    let res = A::new(2, None);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, vec![1]);

    let res2 = A::new(2, Some(vec![6u32]));
    assert_eq!(res2.0, 2);
    assert_eq!(res2.1, vec![6]);
}

#[test]
fn struct_optional_function() {
    fn custom_default() -> u64 {
        3
    }

    #[derive(New)]
    struct A {
        x: u32,
        #[new(default = custom_default(), optional)]
        y: u64,
    }

    let res = A::new(2, None);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, 3);

    let res2 = A::new(2, Some(6u64));
    assert_eq!(res2.x, 2);
    assert_eq!(res2.y, 6);
}

#[test]
fn const_struct_optional_function() {
    const fn custom_default() -> u64 {
        3
    }

    #[derive(New)]
    #[new(const)]
    struct A {
        x: u32,
        #[new(default = custom_default(), optional)]
        y: u64,
    }

    const RES: A = A::new(2, None);
    assert_eq!(RES.x, 2);
    assert_eq!(RES.y, 3);

    const RES2: A = A::new(2, Some(6u64));
    assert_eq!(RES2.x, 2);
    assert_eq!(RES2.y, 6);
}

#[test]
fn tuple_struct_optional_function() {
    fn custom_default() -> u64 {
        3
    }

    #[derive(New)]
    struct A(u32, #[new(default = custom_default(), optional)] u64);

    let res = A::new(2, None);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 3);

    let res2 = A::new(2, Some(6u64));
    assert_eq!(res2.0, 2);
    assert_eq!(res2.1, 6);
}

#[test]
fn const_tuple_struct_optional_function() {
    const fn custom_default() -> u64 {
        3
    }

    #[derive(New)]
    #[new(const)]
    struct A(u32, #[new(default = custom_default(), optional)] u64);

    const RES: A = A::new(2, None);
    assert_eq!(RES.0, 2);
    assert_eq!(RES.1, 3);

    const RES2: A = A::new(2, Some(6u64));
    assert_eq!(RES2.0, 2);
    assert_eq!(RES2.1, 6);
}

#[test]
fn struct_with_nested_optional_function() {
    mod nested {
        pub fn custom_default() -> u64 {
            3
        }
    }

    #[derive(New)]
    struct A {
        x: u32,
        #[new(default = nested::custom_default(), optional)]
        y: u64,
    }

    let res = A::new(2, None);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, 3);

    let res2 = A::new(2, Some(6u64));
    assert_eq!(res2.x, 2);
    assert_eq!(res2.y, 6);
}

#[test]
fn const_struct_with_nested_optional_function() {
    mod nested {
        pub const fn custom_default() -> u64 {
            3
        }
    }

    #[derive(New)]
    #[new(const)]
    struct A {
        x: u32,
        #[new(default = nested::custom_default(), optional)]
        y: u64,
    }

    const RES: A = A::new(2, None);
    assert_eq!(RES.x, 2);
    assert_eq!(RES.y, 3);

    const RES2: A = A::new(2, Some(6u64));
    assert_eq!(RES2.x, 2);
    assert_eq!(RES2.y, 6);
}

#[test]
fn tuple_struct_with_nested_optional_function() {
    mod nested {
        pub fn custom_default() -> u64 {
            3
        }
    }

    #[derive(New)]
    struct A(
        u32,
        #[new(default = nested::custom_default(), optional)] u64,
    );

    let res = A::new(2, None);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 3);

    let res2 = A::new(2, Some(6u64));
    assert_eq!(res2.0, 2);
    assert_eq!(res2.1, 6);
}

#[test]
fn const_tuple_struct_with_nested_optional_function() {
    mod nested {
        pub const fn custom_default() -> u64 {
            3
        }
    }

    #[derive(New)]
    #[new(const)]
    struct A(
        u32,
        #[new(default = nested::custom_default(), optional)] u64,
    );

    const RES: A = A::new(2, None);
    assert_eq!(RES.0, 2);
    assert_eq!(RES.1, 3);

    const RES2: A = A::new(2, Some(6u64));
    assert_eq!(RES2.0, 2);
    assert_eq!(RES2.1, 6);
}

#[test]
fn struct_with_all_optional_defaults() {
    #[derive(New)]
    struct A {
        #[new(optional)]
        x: u32,
        #[new(optional)]
        y: u64,
    }

    let res = A::new(None, None);
    assert_eq!(res.x, 0);
    assert_eq!(res.y, 0);

    let res2 = A::new(Some(6u32), Some(8u64));
    assert_eq!(res2.x, 6);
    assert_eq!(res2.y, 8);
}

#[test]
fn tuple_struct_with_all_optional_defaults() {
    #[derive(New)]
    struct A(#[new(optional)] u32, #[new(optional)] u64);

    let res = A::new(None, None);
    assert_eq!(res.0, 0);
    assert_eq!(res.1, 0);

    let res2 = A::new(Some(6u32), Some(8u64));
    assert_eq!(res2.0, 6);
    assert_eq!(res2.1, 8);
}

#[test]
fn struct_with_mixed_all_optional_defaults() {
    fn custom_default() -> u8 {
        5
    }

    #[derive(New)]
    struct A {
        #[new(optional)]
        x: u32,
        #[new(default = 1 + 2, optional)]
        y: u64,
        #[new(default = custom_default(), optional)]
        z: u8,
    }

    let res = A::new(None, None, None);
    assert_eq!(res.x, 0);
    assert_eq!(res.y, 3);
    assert_eq!(res.z, 5);

    let res2 = A::new(Some(6u32), Some(8u64), Some(10u8));
    assert_eq!(res2.x, 6);
    assert_eq!(res2.y, 8);
    assert_eq!(res2.z, 10);
}

#[test]
fn tuple_struct_with_mixed_all_optional_defaults() {
    fn custom_default() -> u8 {
        5
    }

    #[derive(New)]
    struct A(
        #[new(optional)] u32,
        #[new(default = 1 + 2, optional)] u64,
        #[new(default = custom_default(), optional)] u8,
    );

    let res = A::new(None, None, None);
    assert_eq!(res.0, 0);
    assert_eq!(res.1, 3);
    assert_eq!(res.2, 5);

    let res2 = A::new(Some(6u32), Some(8u64), Some(10u8));
    assert_eq!(res2.0, 6);
    assert_eq!(res2.1, 8);
    assert_eq!(res2.2, 10);
}
