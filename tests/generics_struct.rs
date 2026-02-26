use inew::New;

#[test]
fn struct_with_single_generic() {
    #[derive(New)]
    struct A<T> {
        x: T,
    }

    let res = A::new(1);
    assert_eq!(res.x, 1);
}

#[test]
fn const_struct_with_single_generic() {
    #[derive(New)]
    #[new(const)]
    struct A<T> {
        x: T,
    }

    const RES: A<u32> = A::new(1);
    assert_eq!(RES.x, 1);
}

#[test]
fn tuple_struct_with_single_generic() {
    #[derive(New)]
    struct A<T>(T);

    let res = A::new(1);
    assert_eq!(res.0, 1);
}

#[test]
fn const_tuple_struct_with_single_generic() {
    #[derive(New)]
    #[new(const)]
    struct A<T>(T);

    const RES: A<u32> = A::new(1);
    assert_eq!(RES.0, 1);
}

#[test]
fn struct_with_single_generic_and_another_field() {
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
fn const_struct_with_single_generic_and_another_field() {
    #[derive(New)]
    #[new(const)]
    struct A<T> {
        x: T,
        y: u32,
    }

    const RES: A<u32> = A::new(1, 2);
    assert_eq!(RES.x, 1);
    assert_eq!(RES.y, 2);
}

#[test]
fn tuple_struct_with_single_generic_and_another_field() {
    #[derive(New)]
    struct A<T>(T, u32);

    let res = A::new(1, 2);
    assert_eq!(res.0, 1);
    assert_eq!(res.1, 2);
}

#[test]
fn const_tuple_struct_with_single_generic_and_another_field() {
    #[derive(New)]
    #[new(const)]
    struct A<T>(T, u32);

    const RES: A<u32> = A::new(1, 2);
    assert_eq!(RES.0, 1);
    assert_eq!(RES.1, 2);
}

#[test]
fn struct_with_multiple_generics() {
    #[derive(New)]
    struct A<X, Y> {
        x: X,
        y: Y,
    }

    let res = A::new(1u32, 2u64);
    assert_eq!(res.x, 1);
    assert_eq!(res.y, 2);
}

#[test]
fn const_struct_with_multiple_generics() {
    #[derive(New)]
    #[new(const)]
    struct A<X, Y> {
        x: X,
        y: Y,
    }

    const RES: A<u32, u64> = A::new(1u32, 2u64);
    assert_eq!(RES.x, 1);
    assert_eq!(RES.y, 2);
}

#[test]
fn tuple_struct_with_multiple_generics() {
    #[derive(New)]
    struct A<X, Y>(X, Y);

    let res = A::new(1u32, 2u64);
    assert_eq!(res.0, 1);
    assert_eq!(res.1, 2);
}

#[test]
fn const_tuple_struct_with_multiple_generics() {
    #[derive(New)]
    #[new(const)]
    struct A<X, Y>(X, Y);

    const RES: A<u32, u64> = A::new(1u32, 2u64);
    assert_eq!(RES.0, 1);
    assert_eq!(RES.1, 2);
}

#[test]
fn struct_with_nested_generics() {
    #[derive(New)]
    struct X<Y, Z> {
        y: Y,
        z: Z,
    }

    #[derive(New)]
    struct A<Y, Z> {
        x: X<Y, Z>,
    }

    let res = A::new(X::new(1, "z"));
    assert_eq!(res.x.y, 1);
    assert_eq!(res.x.z, "z");
}

#[test]
fn const_struct_with_nested_generics() {
    #[derive(New)]
    #[new(const)]
    struct X<Y, Z> {
        y: Y,
        z: Z,
    }

    #[derive(New)]
    #[new(const)]
    struct A<Y, Z> {
        x: X<Y, Z>,
    }

    const RES: A<u32, &str> = A::new(X::new(1, "z"));
    assert_eq!(RES.x.y, 1);
    assert_eq!(RES.x.z, "z");
}

#[test]
fn tuple_struct_with_nested_generics() {
    #[derive(New)]
    struct X<Y, Z>(Y, Z);

    #[derive(New)]
    struct A<Y, Z>(X<Y, Z>);

    let res = A::new(X::new(1, "z"));
    assert_eq!(res.0 .0, 1);
    assert_eq!(res.0 .1, "z");
}

#[test]
fn const_tuple_struct_with_nested_generics() {
    #[derive(New)]
    #[new(const)]
    struct X<Y, Z>(Y, Z);

    #[derive(New)]
    #[new(const)]
    struct A<Y, Z>(X<Y, Z>);

    const RES: A<u32, &str> = A::new(X::new(1, "z"));
    assert_eq!(RES.0 .0, 1);
    assert_eq!(RES.0 .1, "z");
}

#[test]
fn struct_with_default_generics() {
    #[derive(New)]
    struct A<X, Y: Default> {
        x: X,
        #[new(default)]
        y: Y,
    }

    let res = A::<u32, u64>::new(1u32);
    assert_eq!(res.x, 1);
    assert_eq!(res.y, 0);
}

#[test]
fn tuple_struct_with_default_generics() {
    #[derive(New)]
    struct A<X, Y: Default>(X, #[new(default)] Y);

    let res = A::<u32, u64>::new(1u32);
    assert_eq!(res.0, 1);
    assert_eq!(res.1, 0);
}
