use inew::New;

#[test]
fn struct_into() {
    #[derive(New)]
    struct A {
        #[new(into)]
        x: String,
    }

    let res = A::new("abc");
    assert_eq!(res.x, "abc");
}

#[test]
fn tuple_struct_into() {
    #[derive(New)]
    struct A(#[new(into)] String);

    let res = A::new("abc");
    assert_eq!(res.0, "abc");
}
