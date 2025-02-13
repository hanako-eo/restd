#[test]
fn launch_extansion_tests() {
    macrotest::expand("tests/expand/*.rs");
}
