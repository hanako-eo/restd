#[test]
fn launch_compilation_tests() {
    let cases = trybuild::TestCases::new();
    cases.pass("tests/ui/success/*.rs");
    cases.compile_fail("tests/ui/fail/*.rs");
}

#[test]
fn launch_extansion_tests() {
    macrotest::expand("tests/expand/*.rs");
}
