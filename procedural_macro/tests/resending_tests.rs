#[test]
fn test_macro_expansion_valid() {
    let t = trybuild::TestCases::new();
    t.pass("tests/fixtures/pass_valid.rs");
    t.compile_fail("tests/fixtures/fail_missing_attr.rs");
    t.compile_fail("tests/fixtures/fail_invalid_value.rs");
}
