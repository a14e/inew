use trybuild::TestCases;

#[test]
fn fail_crate() {
    let tests = TestCases::new();
    tests.compile_fail("fail_crate/tests/*.rs");
}
