use inew::New;

#[test]
fn enum_into_iter() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            #[new(into_iter)]
            x: Vec<u32>,
        },
    }

    let res = A::new_i(Some(5));
    assert_eq!(res, A::I { x: vec![5] });

    let res2 = A::new_i(None);
    assert_eq!(res2, A::I { x: vec![] });

    let res3 = A::new_i([1, 2, 3]);
    assert_eq!(res3, A::I { x: vec![1, 2, 3] });
}

#[test]
fn tuple_enum_into_iter() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(#[new(into_iter)] Vec<u32>),
    }

    let res = A::new_i(Some(5));
    assert_eq!(res, A::I(vec![5]));

    let res2 = A::new_i(None);
    assert_eq!(res2, A::I(vec![]));

    let res3 = A::new_i([1, 2, 3]);
    assert_eq!(res3, A::I(vec![1, 2, 3]));
}

#[test]
fn enum_type_alias_into_iter() {
    type X = Vec<u32>;

    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            #[new(into_iter = u32)]
            x: X,
        },
    }

    let res = A::new_i(Some(5));
    assert_eq!(res, A::I { x: vec![5] });
}

#[test]
fn tuple_enum_type_alias_into_iter() {
    type X = Vec<u32>;

    #[derive(Debug, PartialEq, New)]
    enum A {
        I(#[new(into_iter = u32)] X),
    }

    let res = A::new_i(Some(5));
    assert_eq!(res, A::I(vec![5]));
}

#[test]
fn enum_into_nested_iter() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            #[new(into_iter)]
            x: Vec<Vec<u32>>,
        },
    }

    let res = A::new_i(Some(vec![5]));
    assert_eq!(res, A::I { x: vec![vec![5]] });
}

#[test]
fn tuple_enum_into_nested_iter() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(#[new(into_iter)] Vec<Vec<u32>>),
    }

    let res = A::new_i(Some(vec![5]));
    assert_eq!(res, A::I(vec![vec![5]]));
}

#[test]
fn enum_type_alias_into_nested_iter() {
    type X = Vec<u32>;
    type Y = Vec<X>;

    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            #[new(into_iter = Vec<u32>)]
            y: Y,
        },
    }

    let res = A::new_i(Some(vec![5]));
    assert_eq!(res, A::I { y: vec![vec![5]] });
}

#[test]
fn tuple_enum_type_alias_into_nested_iter() {
    type X = Vec<u32>;
    type Y = Vec<X>;

    #[derive(Debug, PartialEq, New)]
    enum A {
        I(#[new(into_iter = Vec<u32>)] Y),
    }

    let res = A::new_i(Some(vec![5]));
    assert_eq!(res, A::I(vec![vec![5]]));
}
