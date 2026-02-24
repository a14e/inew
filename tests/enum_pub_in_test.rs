mod nested {
    use crate::nested::nested2::{A, B, C, D};

    pub mod nested2 {
        use inew::New;

        #[derive(Debug, PartialEq, New)]
        #[new(pub(in crate::nested))]
        pub enum A {
            I { x: u32 },
        }

        #[derive(Debug, PartialEq, New)]
        #[new(pub(in crate::nested))]
        pub enum B {
            I(u64),
        }

        #[derive(Debug, PartialEq, New)]
        #[new(pub(in crate::nested), const)]
        pub enum C {
            I { x: u32 },
        }

        #[derive(Debug, PartialEq, New)]
        #[new(pub(in crate::nested), const)]
        pub enum D {
            I(u64),
        }
    }

    pub fn test_a() {
        let res = A::new_i(1);
        assert_eq!(res, A::I { x: 1 });
    }

    pub fn test_b() {
        let res = B::new_i(2);
        assert_eq!(res, B::I(2));
    }

    pub fn test_c() {
        const RES: C = C::new_i(3);
        assert_eq!(RES, C::I { x: 3 });
    }

    pub fn test_d() {
        const RES: D = D::new_i(4);
        assert_eq!(RES, D::I(4));
    }
}

#[test]
fn enum_pub_in_visibility() {
    nested::test_a();
}

#[test]
fn tuple_enum_pub_in_visibility() {
    nested::test_b();
}

#[test]
fn const_enum_pub_in_visibility() {
    nested::test_c();
}

#[test]
fn const_tuple_enum_pub_in_visibility() {
    nested::test_d();
}
