use inew::New;

#[test]
fn struct_public_new() {
    #[derive(New)]
    #[new(pub)]
    struct A {
        x: u32,
    }

    #[derive(New)]
    #[new(pub = true)]
    struct B {
        x: u32,
    }

    let res = A::new(1);
    assert_eq!(res.x, 1);

    let res2 = B::new(2);
    assert_eq!(res2.x, 2);
}

#[test]
fn const_struct_public_new() {
    #[derive(New)]
    #[new(pub, const)]
    struct A {
        x: u32,
    }

    #[derive(New)]
    #[new(pub = true, const)]
    struct B {
        x: u32,
    }

    const RES: A = A::new(1);
    assert_eq!(RES.x, 1);

    const RES2: B = B::new(2);
    assert_eq!(RES2.x, 2);
}

#[test]
fn tuple_struct_public_new() {
    #[derive(New)]
    #[new(pub)]
    struct A(u32);

    #[derive(New)]
    #[new(pub = true)]
    struct B(u32);

    let res = A::new(1);
    assert_eq!(res.0, 1);

    let res2 = B::new(2);
    assert_eq!(res2.0, 2);
}

#[test]
fn const_tuple_struct_public_new() {
    #[derive(New)]
    #[new(pub, const)]
    struct A(u32);

    #[derive(New)]
    #[new(pub = true, const)]
    struct B(u32);

    const RES: A = A::new(1);
    assert_eq!(RES.0, 1);

    const RES2: B = B::new(2);
    assert_eq!(RES2.0, 2);
}

#[test]
fn struct_explicit_private_new() {
    #[derive(New)]
    #[new(pub = false)]
    struct A {
        x: u32,
    }

    let res = A::new(1);
    assert_eq!(res.x, 1);
}

#[test]
fn const_struct_explicit_private_new() {
    #[derive(New)]
    #[new(pub = false, const)]
    struct A {
        x: u32,
    }

    const RES: A = A::new(1);
    assert_eq!(RES.x, 1);
}

#[test]
fn tuple_struct_explicit_private_new() {
    #[derive(New)]
    #[new(pub = false)]
    struct A(u32);

    let res = A::new(1);
    assert_eq!(res.0, 1);
}

#[test]
fn const_tuple_struct_explicit_private_new() {
    #[derive(New)]
    #[new(pub = false, const)]
    struct A(u32);

    const RES: A = A::new(1);
    assert_eq!(RES.0, 1);
}

#[test]
fn struct_custom_visibility() {
    #[derive(New)]
    #[new(pub(crate))]
    struct A {
        x: u32,
    }

    mod nested {
        use super::*;

        #[derive(New)]
        #[new(pub(super))]
        pub struct B {
            pub x: u32,
        }
    }

    #[derive(New)]
    #[new(pub(self))]
    struct C {
        x: u32,
    }

    let res = A::new(1);
    assert_eq!(res.x, 1);

    let res2 = nested::B::new(2);
    assert_eq!(res2.x, 2);

    let res3 = C::new(3);
    assert_eq!(res3.x, 3);
}

#[test]
fn const_struct_custom_visibility() {
    #[derive(New)]
    #[new(pub(crate), const)]
    struct A {
        x: u32,
    }

    mod nested {
        use super::*;

        #[derive(New)]
        #[new(pub(super), const)]
        pub struct B {
            pub x: u32,
        }
    }

    #[derive(New)]
    #[new(pub(self), const)]
    struct C {
        x: u32,
    }

    const RES: A = A::new(1);
    assert_eq!(RES.x, 1);

    const RES2: nested::B = nested::B::new(2);
    assert_eq!(RES2.x, 2);

    const RES3: C = C::new(3);
    assert_eq!(RES3.x, 3);
}

#[test]
fn tuple_struct_custom_visibility() {
    #[derive(New)]
    #[new(pub(crate))]
    struct A(u32);

    mod nested {
        use super::*;

        #[derive(New)]
        #[new(pub(super))]
        pub struct B(pub u32);
    }

    #[derive(New)]
    #[new(pub(self))]
    struct C(u32);

    let res = A::new(1);
    assert_eq!(res.0, 1);

    let res2 = nested::B::new(2);
    assert_eq!(res2.0, 2);

    let res3 = C::new(3);
    assert_eq!(res3.0, 3);
}

#[test]
fn const_tuple_struct_custom_visibility() {
    #[derive(New)]
    #[new(pub(crate), const)]
    struct A(u32);

    mod nested {
        use super::*;

        #[derive(New)]
        #[new(pub(super), const)]
        pub struct B(pub u32);
    }

    #[derive(New)]
    #[new(pub(self), const)]
    struct C(u32);

    const RES: A = A::new(1);
    assert_eq!(RES.0, 1);

    const RES2: nested::B = nested::B::new(2);
    assert_eq!(RES2.0, 2);

    const RES3: C = C::new(3);
    assert_eq!(RES3.0, 3);
}
