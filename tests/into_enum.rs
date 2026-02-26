use inew::New;

#[test]
fn enum_into() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I {
            #[new(into)]
            x: String,
        },
    }

    let res = A::new_i("abc");
    assert_eq!(
        res,
        A::I {
            x: "abc".to_string()
        }
    );
}

#[test]
fn tuple_enum_into() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        I(#[new(into)] String),
    }

    let res = A::new_i("abc");
    assert_eq!(res, A::I("abc".to_string()));
}
