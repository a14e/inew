#[cfg(feature = "std")]
use trybuild::TestCases;

#[cfg(feature = "std")]
#[test]
fn fail_crate() {
    let tests = TestCases::new();
    tests.compile_fail("fail_crate/tests/*.rs");
}
