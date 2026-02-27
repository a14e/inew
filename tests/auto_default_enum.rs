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
fn enum_std_phantom_data_auto_default() {
    #[derive(Debug, PartialEq, New)]
    enum A<T> {
        I { x: std::marker::PhantomData<T> },
    }

    let res = A::<u32>::new_i();
    assert_eq!(res, A::I { x: PhantomData });
}

#[test]
fn const_enum_std_phantom_data_auto_default() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A<T> {
        I { x: std::marker::PhantomData<T> },
    }

    const RES: A<u32> = A::new_i();
    assert_eq!(RES, A::I { x: PhantomData });
}

#[test]
fn tuple_enum_std_phantom_data_auto_default() {
    #[derive(Debug, PartialEq, New)]
    enum A<T> {
        I(std::marker::PhantomData<T>),
    }

    let res = A::<u32>::new_i();
    assert_eq!(res, A::I(PhantomData));
}

#[test]
fn const_tuple_enum_std_phantom_data_auto_default() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A<T> {
        I(std::marker::PhantomData<T>),
    }

    const RES: A<u32> = A::new_i();
    assert_eq!(RES, A::I(PhantomData));
}

#[test]
fn enum_core_phantom_data_auto_default() {
    #[derive(Debug, PartialEq, New)]
    enum A<T> {
        I { x: core::marker::PhantomData<T> },
    }

    let res = A::<u32>::new_i();
    assert_eq!(res, A::I { x: PhantomData });
}

#[test]
fn const_enum_core_phantom_data_auto_default() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A<T> {
        I { x: core::marker::PhantomData<T> },
    }

    const RES: A<u32> = A::new_i();
    assert_eq!(RES, A::I { x: PhantomData });
}

#[test]
fn tuple_enum_core_phantom_data_auto_default() {
    #[derive(Debug, PartialEq, New)]
    enum A<T> {
        I(core::marker::PhantomData<T>),
    }

    let res = A::<u32>::new_i();
    assert_eq!(res, A::I(PhantomData));
}

#[test]
fn const_tuple_enum_core_phantom_data_auto_default() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A<T> {
        I(core::marker::PhantomData<T>),
    }

    const RES: A<u32> = A::new_i();
    assert_eq!(RES, A::I(PhantomData));
}
