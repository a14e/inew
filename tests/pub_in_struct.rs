#![cfg_attr(not(feature = "std"), no_std)]

mod nested {
    use crate::nested::nested2::{A, B, C, D};

    pub mod nested2 {
        use inew::New;

        #[derive(New)]
        #[new(pub(in crate::nested))]
        pub struct A {
            pub x: u32,
        }

        #[derive(New)]
        #[new(pub(in crate::nested), const)]
        pub struct B {
            pub x: u32,
        }

        #[derive(New)]
        #[new(pub(in crate::nested))]
        pub struct C(pub u64);

        #[derive(New)]
        #[new(pub(in crate::nested), const)]
        pub struct D(pub u64);
    }

    pub fn test_a() {
        let res = A::new(1);
        assert_eq!(res.x, 1);
    }

    pub fn test_b() {
        const RES: B = B::new(2);
        assert_eq!(RES.x, 2);
    }

    pub fn test_c() {
        let res = C::new(3);
        assert_eq!(res.0, 3);
    }

    pub fn test_d() {
        const RES: D = D::new(4);
        assert_eq!(RES.0, 4);
    }
}

#[test]
fn struct_pub_in_visibility() {
    nested::test_a();
}

#[test]
fn const_struct_pub_in_visibility() {
    nested::test_b();
}

#[test]
fn tuple_struct_pub_in_visibility() {
    nested::test_c();
}

#[test]
fn const_tuple_struct_pub_in_visibility() {
    nested::test_d();
}
