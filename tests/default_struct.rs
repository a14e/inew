use inew::New;

#[test]
fn struct_with_default() {
    #[derive(New)]
    struct A {
        x: u32,
        #[new(default)]
        y: u64,
    }

    let res = A::new(2);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, 0);
}

#[test]
fn tuple_struct_with_default() {
    #[derive(New)]
    struct A(u32, #[new(default)] u64);

    let res = A::new(2);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 0);
}

#[test]
fn struct_with_default_expression() {
    #[derive(New)]
    struct A {
        x: u32,
        #[new(default = 1 + 2)]
        y: u64,
    }

    let res = A::new(2);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, 3);
}

#[test]
fn const_struct_with_default_expression() {
    #[derive(New)]
    #[new(const)]
    struct A {
        x: u32,
        #[new(default = 1 + 2)]
        y: u64,
    }

    const RES: A = A::new(2);
    assert_eq!(RES.x, 2);
    assert_eq!(RES.y, 3);
}

#[test]
fn tuple_struct_with_default_expression() {
    #[derive(New)]
    struct A(u32, #[new(default = 1 + 2)] u64);

    let res = A::new(2);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 3);
}

#[test]
fn const_tuple_struct_with_default_expression() {
    #[derive(New)]
    #[new(const)]
    struct A(u32, #[new(default = 1 + 2)] u64);

    const RES: A = A::new(2);
    assert_eq!(RES.0, 2);
    assert_eq!(RES.1, 3);
}

#[test]
fn struct_with_default_custom_macro() {
    macro_rules! custom_macro {
        () => {
            7
        };
    }

    #[derive(New)]
    struct A {
        x: u32,
        #[new(default = custom_macro!())]
        y: u64,
    }

    let res = A::new(2);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, 7);
}

#[test]
fn const_struct_with_default_custom_macro() {
    macro_rules! custom_macro {
        () => {
            7
        };
    }

    #[derive(New)]
    #[new(const)]
    struct A {
        x: u32,
        #[new(default = custom_macro!())]
        y: u64,
    }

    const RES: A = A::new(2);
    assert_eq!(RES.x, 2);
    assert_eq!(RES.y, 7);
}

#[test]
fn tuple_struct_with_default_custom_macro() {
    macro_rules! custom_macro {
        () => {
            7
        };
    }

    #[derive(New)]
    struct A(u32, #[new(default = custom_macro!())] u64);

    let res = A::new(2);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 7);
}

#[test]
fn const_tuple_struct_with_default_custom_macro() {
    macro_rules! custom_macro {
        () => {
            7
        };
    }

    #[derive(New)]
    #[new(const)]
    struct A(u32, #[new(default = custom_macro!())] u64);

    const RES: A = A::new(2);
    assert_eq!(RES.0, 2);
    assert_eq!(RES.1, 7);
}

#[test]
fn struct_with_default_allocation_macro() {
    #[derive(New)]
    struct A {
        x: u32,
        #[new(default = vec![ 1u32 ])]
        y: Vec<u32>,
    }

    let res = A::new(2);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, vec![1]);
}

#[test]
fn tuple_struct_with_default_allocation_macro() {
    #[derive(New)]
    struct A(u32, #[new(default = vec![ 1u32 ])] Vec<u32>);

    let res = A::new(2);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, vec![1]);
}

#[test]
fn struct_with_default_function() {
    fn custom_default() -> u64 {
        3
    }

    #[derive(New)]
    struct A {
        x: u32,
        #[new(default = custom_default())]
        y: u64,
    }

    let res = A::new(2);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, 3);
}

#[test]
fn const_struct_with_default_function() {
    const fn custom_default() -> u64 {
        3
    }

    #[derive(New)]
    #[new(const)]
    struct A {
        x: u32,
        #[new(default = custom_default())]
        y: u64,
    }

    const RES: A = A::new(2);
    assert_eq!(RES.x, 2);
    assert_eq!(RES.y, 3);
}

#[test]
fn tuple_struct_with_default_function() {
    fn custom_default() -> u64 {
        3
    }

    #[derive(New)]
    struct A(u32, #[new(default = custom_default())] u64);

    let res = A::new(2);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 3);
}

#[test]
fn const_tuple_struct_with_default_function() {
    const fn custom_default() -> u64 {
        3
    }

    #[derive(New)]
    #[new(const)]
    struct A(u32, #[new(default = custom_default())] u64);

    const RES: A = A::new(2);
    assert_eq!(RES.0, 2);
    assert_eq!(RES.1, 3);
}

#[test]
fn struct_with_nested_default_function() {
    mod nested {
        pub fn custom_default() -> u64 {
            3
        }
    }

    #[derive(New)]
    struct A {
        x: u32,
        #[new(default = nested::custom_default())]
        y: u64,
    }

    let res = A::new(2);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, 3);
}

#[test]
fn const_struct_with_nested_default_function() {
    mod nested {
        pub const fn custom_default() -> u64 {
            3
        }
    }

    #[derive(New)]
    #[new(const)]
    struct A {
        x: u32,
        #[new(default = nested::custom_default())]
        y: u64,
    }

    const RES: A = A::new(2);
    assert_eq!(RES.x, 2);
    assert_eq!(RES.y, 3);
}

#[test]
fn tuple_struct_with_nested_default_function() {
    mod nested {
        pub fn custom_default() -> u64 {
            3
        }
    }

    #[derive(New)]
    struct A(u32, #[new(default = nested::custom_default())] u64);

    let res = A::new(2);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 3);
}

#[test]
fn const_tuple_struct_with_nested_default_function() {
    mod nested {
        pub const fn custom_default() -> u64 {
            3
        }
    }

    #[derive(New)]
    #[new(const)]
    struct A(u32, #[new(default = nested::custom_default())] u64);

    const RES: A = A::new(2);
    assert_eq!(RES.0, 2);
    assert_eq!(RES.1, 3);
}

#[test]
fn struct_with_all_defaults() {
    #[derive(New)]
    struct A {
        #[new(default)]
        x: u32,
        #[new(default)]
        y: u64,
    }

    let res = A::new();
    assert_eq!(res.x, 0);
    assert_eq!(res.y, 0);
}

#[test]
fn tuple_struct_with_all_defaults() {
    #[derive(New)]
    struct A(#[new(default)] u32, #[new(default)] u64);

    let res = A::new();
    assert_eq!(res.0, 0);
    assert_eq!(res.1, 0);
}

#[test]
fn struct_with_mixed_all_defaults() {
    fn custom_default() -> u8 {
        5
    }

    #[derive(New)]
    struct A {
        #[new(default)]
        x: u32,
        #[new(default = 1 + 2)]
        y: u64,
        #[new(default = custom_default())]
        z: u8,
    }

    let res = A::new();
    assert_eq!(res.x, 0);
    assert_eq!(res.y, 3);
    assert_eq!(res.z, 5);
}

#[test]
fn tuple_struct_with_mixed_all_defaults() {
    fn custom_default() -> u8 {
        5
    }

    #[derive(New)]
    struct A(
        #[new(default)] u32,
        #[new(default = 1 + 2)] u64,
        #[new(default = custom_default())] u8,
    );

    let res = A::new();
    assert_eq!(res.0, 0);
    assert_eq!(res.1, 3);
    assert_eq!(res.2, 5);
}
