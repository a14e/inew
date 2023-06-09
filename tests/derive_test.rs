use std::marker::PhantomData;
use inew::New;


#[test]
fn basic_new_single_field() {
    #[derive(New)]
    struct A {
        x: u32,
    }
    let res = A::new(1);
    assert_eq!(res.x, 1)
}

#[test]
fn basic_new_zero_field() {
    #[derive(New)]
    struct A {}
    let _ = A::new();
}

#[test]
fn basic_new_without_braces() {
    #[derive(New)]
    struct A;
    let _ = A::new();
}

#[test]
fn basic_new_multiple_fields() {
    #[derive(New)]
    struct A {
        x: u32,
        y: u64,
    }
    let res = A::new(2, 3);
    assert_eq!(res.x, 2);
    assert_eq!(res.y, 3);
}

#[test]
fn unit_support() {
    #[derive(New)]
    struct A {
        _x: (),
    }
    let _ = A::new();
}

#[test]
fn phantom_support() {
    #[derive(New)]
    struct A<T> {
        x: PhantomData<T>,
    }
    let _: A<u32> = A::new();
}


#[test]
fn new_with_default() {
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
fn new_with_default_func() {
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
fn new_with_default_func_nested() {
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
fn new_with_default_expr() {

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
fn unnamed_struct() {

    #[derive(New)]
    struct A(u32);

    let res = A::new(2);
    assert_eq!(res.0, 2);
}

#[test]
fn unnamed_struct_default() {

    #[derive(New)]
    struct A(#[new(default)]u32);

    let _res = A::new();
}

#[test]
fn new_with_default_expr_macro() {

    #[derive(New)]
    struct A {
        x: u32,
        #[new(default = vec![ 1u32 ])]
        _y: Vec<u32>,
    }
    let res = A::new(2);
    assert_eq!(res.x, 2);
}


#[test]
fn new_with_all_default() {
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
fn new_with_generics_with_single_field() {
    #[derive(New)]
    struct A<T> {
        x: T,
    }
    let res = A::new(1);
    assert_eq!(res.x, 1);
}

#[test]
fn new_with_generics_with_multiple_fields() {
    #[derive(New)]
    struct A<T> {
        x: T,
        y: u32,
    }
    let res = A::new(1, 2);
    assert_eq!(res.x, 1);
    assert_eq!(res.y, 2);
}

#[test]
fn new_with_generics_with_multiple_generics() {
    #[derive(New)]
    struct A<C, B> {
        x: C,
        y: B,
    }
    let res = A::new(1u32, 2u64);
    assert_eq!(res.x, 1);
    assert_eq!(res.y, 2);
}

#[test]
fn new_with_generics_and_defaults() {
    #[derive(New)]
    struct A<C, B: Default> {
        x: C,
        #[new(default)]
        y: B,
    }
    let res = A::<_, u64>::new(1u32);
    assert_eq!(res.x, 1);
    assert_eq!(res.y, 0);
}


#[test]
fn new_with_lifetimes() {
    #[derive(New)]
    struct A<'a> {
        x: &'a u64,
    }
    let x = 1u64;
    let res = A::new(&x);
    assert_eq!(*res.x, 1);
}


#[test]
fn new_with_lifetimes_and_generics() {
    #[derive(New)]
    struct A<'a, T> {
        x: &'a T,
    }
    let x = 1u64;
    let res = A::new(&x);
    assert_eq!(*res.x, 1);
}


#[test]
fn private_field() {
    #[derive(New)]
    #[new(pub = false)]
    struct A<'a, T> {
        x: &'a T,
    }
    let x = 1u64;
    let res = A::new(&x);
    assert_eq!(*res.x, 1);
}


#[test]
fn rename_field() {
    #[derive(New)]
    #[new(rename = "create")]
    struct A<'a, T> {
        x: &'a T,
    }
    let x = 1u64;
    let res = A::create(&x);
    assert_eq!(*res.x, 1);
}