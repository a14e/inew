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
        #[new(pub(in crate::nested), const)]
        pub enum B {
            I { x: u32 },
        }

        #[derive(Debug, PartialEq, New)]
        #[new(pub(in crate::nested))]
        pub enum C {
            I(u64),
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
        const RES: B = B::new_i(2);
        assert_eq!(RES, B::I { x: 2 });
    }

    pub fn test_c() {
        let res = C::new_i(3);
        assert_eq!(res, C::I(3));
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
fn const_enum_pub_in_visibility() {
    nested::test_b();
}

#[test]
fn tuple_enum_pub_in_visibility() {
    nested::test_c();
}

#[test]
fn const_tuple_enum_pub_in_visibility() {
    nested::test_d();
}
