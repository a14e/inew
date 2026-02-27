#![cfg_attr(not(feature = "std"), no_std)]

use inew::New;

#[test]
fn enum_rename_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(rename = "create")]
    enum A {
        I { x: u32 },
    }

    let res = A::create_i(1);
    assert_eq!(res, A::I { x: 1 });
}

#[test]
fn const_enum_rename_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(rename = "create", const)]
    enum A {
        I { x: u32 },
    }

    const RES: A = A::create_i(1);
    assert_eq!(RES, A::I { x: 1 });
}

#[test]
fn tuple_enum_rename_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(rename = "create")]
    enum A {
        I(u32),
    }

    let res = A::create_i(1);
    assert_eq!(res, A::I(1));
}

#[test]
fn const_tuple_enum_rename_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(rename = "create", const)]
    enum A {
        I(u32),
    }

    const RES: A = A::create_i(1);
    assert_eq!(RES, A::I(1));
}

#[test]
fn enum_constructor_name() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        HTTPStatus { x: u32 },
        XMLHttpRequest { y: u64 },
        SimpleURLParser { z: u8 },
        XYZ { w: u16 },
    }

    let res = A::new_http_status(1);
    assert_eq!(res, A::HTTPStatus { x: 1 });

    let res2 = A::new_xml_http_request(2);
    assert_eq!(res2, A::XMLHttpRequest { y: 2 });

    let res3 = A::new_simple_url_parser(3);
    assert_eq!(res3, A::SimpleURLParser { z: 3 });

    let res4 = A::new_xyz(4);
    assert_eq!(res4, A::XYZ { w: 4 });
}

#[test]
fn const_enum_constructor_name() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        HTTPStatus { x: u32 },
        XMLHttpRequest { y: u64 },
        SimpleURLParser { z: u8 },
        XYZ { w: u16 },
    }

    const RES: A = A::new_http_status(1);
    assert_eq!(RES, A::HTTPStatus { x: 1 });

    const RES2: A = A::new_xml_http_request(2);
    assert_eq!(RES2, A::XMLHttpRequest { y: 2 });

    const RES3: A = A::new_simple_url_parser(3);
    assert_eq!(RES3, A::SimpleURLParser { z: 3 });

    const RES4: A = A::new_xyz(4);
    assert_eq!(RES4, A::XYZ { w: 4 });
}

#[test]
fn tuple_enum_constructor_name() {
    #[derive(Debug, PartialEq, New)]
    enum A {
        HTTPStatus(u32),
        XMLHttpRequest(u64),
        SimpleURLParser(u8),
        XYZ(u16),
    }

    let res = A::new_http_status(1);
    assert_eq!(res, A::HTTPStatus(1));

    let res2 = A::new_xml_http_request(2);
    assert_eq!(res2, A::XMLHttpRequest(2));

    let res3 = A::new_simple_url_parser(3);
    assert_eq!(res3, A::SimpleURLParser(3));

    let res4 = A::new_xyz(4);
    assert_eq!(res4, A::XYZ(4));
}

#[test]
fn const_tuple_enum_constructor_name() {
    #[derive(Debug, PartialEq, New)]
    #[new(const)]
    enum A {
        HTTPStatus(u32),
        XMLHttpRequest(u64),
        SimpleURLParser(u8),
        XYZ(u16),
    }

    const RES: A = A::new_http_status(1);
    assert_eq!(RES, A::HTTPStatus(1));

    const RES2: A = A::new_xml_http_request(2);
    assert_eq!(RES2, A::XMLHttpRequest(2));

    const RES3: A = A::new_simple_url_parser(3);
    assert_eq!(RES3, A::SimpleURLParser(3));

    const RES4: A = A::new_xyz(4);
    assert_eq!(RES4, A::XYZ(4));
}

#[test]
fn enum_constructor_rename_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(rename = "create")]
    enum A {
        HTTPStatus { x: u32 },
        XMLHttpRequest { y: u64 },
        SimpleURLParser { z: u8 },
        XYZ { w: u16 },
    }

    let res = A::create_http_status(1);
    assert_eq!(res, A::HTTPStatus { x: 1 });

    let res2 = A::create_xml_http_request(2);
    assert_eq!(res2, A::XMLHttpRequest { y: 2 });

    let res3 = A::create_simple_url_parser(3);
    assert_eq!(res3, A::SimpleURLParser { z: 3 });

    let res4 = A::create_xyz(4);
    assert_eq!(res4, A::XYZ { w: 4 });
}

#[test]
fn const_enum_constructor_rename_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(rename = "create", const)]
    enum A {
        HTTPStatus { x: u32 },
        XMLHttpRequest { y: u64 },
        SimpleURLParser { z: u8 },
        XYZ { w: u16 },
    }

    const RES: A = A::create_http_status(1);
    assert_eq!(RES, A::HTTPStatus { x: 1 });

    const RES2: A = A::create_xml_http_request(2);
    assert_eq!(RES2, A::XMLHttpRequest { y: 2 });

    const RES3: A = A::create_simple_url_parser(3);
    assert_eq!(RES3, A::SimpleURLParser { z: 3 });

    const RES4: A = A::create_xyz(4);
    assert_eq!(RES4, A::XYZ { w: 4 });
}

#[test]
fn tuple_enum_constructor_rename_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(rename = "create")]
    enum A {
        HTTPStatus(u32),
        XMLHttpRequest(u64),
        SimpleURLParser(u8),
        XYZ(u16),
    }

    let res = A::create_http_status(1);
    assert_eq!(res, A::HTTPStatus(1));

    let res2 = A::create_xml_http_request(2);
    assert_eq!(res2, A::XMLHttpRequest(2));

    let res3 = A::create_simple_url_parser(3);
    assert_eq!(res3, A::SimpleURLParser(3));

    let res4 = A::create_xyz(4);
    assert_eq!(res4, A::XYZ(4));
}

#[test]
fn const_tuple_enum_constructor_rename_new() {
    #[derive(Debug, PartialEq, New)]
    #[new(rename = "create", const)]
    enum A {
        HTTPStatus(u32),
        XMLHttpRequest(u64),
        SimpleURLParser(u8),
        XYZ(u16),
    }

    const RES: A = A::create_http_status(1);
    assert_eq!(RES, A::HTTPStatus(1));

    const RES2: A = A::create_xml_http_request(2);
    assert_eq!(RES2, A::XMLHttpRequest(2));

    const RES3: A = A::create_simple_url_parser(3);
    assert_eq!(RES3, A::SimpleURLParser(3));

    const RES4: A = A::create_xyz(4);
    assert_eq!(RES4, A::XYZ(4));
}

#[test]
fn enum_no_prefix() {
    #[derive(Debug, PartialEq, New)]
    #[new(no_prefix)]
    enum A {
        I { x: u32 },
        J { y: u64 },
        K { z: u8 },
    }

    let res = A::i(1);
    assert_eq!(res, A::I { x: 1 });

    let res2 = A::j(2);
    assert_eq!(res2, A::J { y: 2 });

    let res3 = A::k(3);
    assert_eq!(res3, A::K { z: 3 });
}

#[test]
fn const_enum_no_prefix() {
    #[derive(Debug, PartialEq, New)]
    #[new(no_prefix, const)]
    enum A {
        I { x: u32 },
        J { y: u64 },
        K { z: u8 },
    }

    const RES: A = A::i(1);
    assert_eq!(RES, A::I { x: 1 });

    const RES2: A = A::j(2);
    assert_eq!(RES2, A::J { y: 2 });

    const RES3: A = A::k(3);
    assert_eq!(RES3, A::K { z: 3 });
}

#[test]
fn tuple_enum_no_prefix() {
    #[derive(Debug, PartialEq, New)]
    #[new(no_prefix)]
    enum A {
        I(u32),
        J(u64),
        K(u8),
    }

    let res = A::i(1);
    assert_eq!(res, A::I(1));

    let res2 = A::j(2);
    assert_eq!(res2, A::J(2));

    let res3 = A::k(3);
    assert_eq!(res3, A::K(3));
}

#[test]
fn const_tuple_enum_no_prefix() {
    #[derive(Debug, PartialEq, New)]
    #[new(no_prefix, const)]
    enum A {
        I(u32),
        J(u64),
        K(u8),
    }

    const RES: A = A::i(1);
    assert_eq!(RES, A::I(1));

    const RES2: A = A::j(2);
    assert_eq!(RES2, A::J(2));

    const RES3: A = A::k(3);
    assert_eq!(RES3, A::K(3));
}

#[test]
fn enum_explicit_no_prefix() {
    #[derive(Debug, PartialEq, New)]
    #[new(no_prefix = true)]
    enum A {
        I { x: u32 },
    }

    #[derive(Debug, PartialEq, New)]
    #[new(no_prefix = false)]
    enum B {
        J { x: u32 },
    }

    let res = A::i(1);
    assert_eq!(res, A::I { x: 1 });

    let res2 = B::new_j(1);
    assert_eq!(res2, B::J { x: 1 });
}

#[test]
fn const_enum_explicit_no_prefix() {
    #[derive(Debug, PartialEq, New)]
    #[new(no_prefix = true, const)]
    enum A {
        I { x: u32 },
    }

    #[derive(Debug, PartialEq, New)]
    #[new(no_prefix = false, const)]
    enum B {
        J { x: u32 },
    }

    const RES: A = A::i(1);
    assert_eq!(RES, A::I { x: 1 });

    const RES2: B = B::new_j(1);
    assert_eq!(RES2, B::J { x: 1 });
}

#[test]
fn tuple_enum_explicit_no_prefix() {
    #[derive(Debug, PartialEq, New)]
    #[new(no_prefix = true)]
    enum A {
        I(u32),
    }

    #[derive(Debug, PartialEq, New)]
    #[new(no_prefix = false)]
    enum B {
        J(u32),
    }

    let res = A::i(1);
    assert_eq!(res, A::I(1));

    let res2 = B::new_j(1);
    assert_eq!(res2, B::J(1));
}

#[test]
fn const_tuple_enum_explicit_no_prefix() {
    #[derive(Debug, PartialEq, New)]
    #[new(no_prefix = true, const)]
    enum A {
        I(u32),
    }

    #[derive(Debug, PartialEq, New)]
    #[new(no_prefix = false, const)]
    enum B {
        J(u32),
    }

    const RES: A = A::i(1);
    assert_eq!(RES, A::I(1));

    const RES2: B = B::new_j(1);
    assert_eq!(RES2, B::J(1));
}
