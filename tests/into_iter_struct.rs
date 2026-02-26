use inew::New;

#[test]
fn struct_into_iter() {
    #[derive(New)]
    struct A {
        #[new(into_iter)]
        x: Vec<u32>,
    }

    let res = A::new(Some(5));
    assert_eq!(res.x, vec![5]);

    let res2 = A::new(None);
    assert_eq!(res2.x, vec![]);

    let res3 = A::new([1, 2, 3]);
    assert_eq!(res3.x, vec![1, 2, 3]);
}

#[test]
fn tuple_struct_into_iter() {
    #[derive(New)]
    struct A(#[new(into_iter)] Vec<u32>);

    let res = A::new(Some(5));
    assert_eq!(res.0, vec![5]);

    let res2 = A::new(None);
    assert_eq!(res2.0, vec![]);

    let res3 = A::new([1, 2, 3]);
    assert_eq!(res3.0, vec![1, 2, 3]);
}

#[test]
fn struct_type_alias_into_iter() {
    type X = Vec<u32>;

    #[derive(New)]
    struct A {
        #[new(into_iter = u32)]
        x: X,
    }

    let res = A::new(Some(5));
    assert_eq!(res.x, vec![5]);
}

#[test]
fn tuple_struct_type_alias_into_iter() {
    type X = Vec<u32>;

    #[derive(New)]
    struct A(#[new(into_iter = u32)] X);

    let res = A::new(Some(5));
    assert_eq!(res.0, vec![5]);
}

#[test]
fn struct_into_nested_iter() {
    #[derive(New)]
    struct A {
        #[new(into_iter)]
        x: Vec<Vec<u32>>,
    }

    let res = A::new(Some(vec![5]));
    assert_eq!(res.x, vec![vec![5]]);
}

#[test]
fn tuple_struct_into_nested_iter() {
    #[derive(New)]
    struct A(#[new(into_iter)] Vec<Vec<u32>>);

    let res = A::new(Some(vec![5]));
    assert_eq!(res.0, vec![vec![5]]);
}

#[test]
fn struct_type_alias_into_nested_iter() {
    type X = Vec<u32>;
    type Y = Vec<X>;

    #[derive(New)]
    struct A {
        #[new(into_iter = Vec<u32>)]
        y: Y,
    }

    let res = A::new(Some(vec![5]));
    assert_eq!(res.y, vec![vec![5]]);
}

#[test]
fn tuple_struct_type_alias_into_nested_iter() {
    type X = Vec<u32>;
    type Y = Vec<X>;

    #[derive(New)]
    struct A(#[new(into_iter = Vec<u32>)] Y);

    let res = A::new(Some(vec![5]));
    assert_eq!(res.0, vec![vec![5]]);
}
