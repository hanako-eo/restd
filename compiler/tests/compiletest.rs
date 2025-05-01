use trybuild::{Args, TestCases};

#[test]
fn launch_compilation_tests() {
    let set_test_items = |args: Args| args.add_cargo_flag(["--features", "test_lang_item"]);

    let cases = TestCases::new();
    cases.pass_args("tests/ui/success/*.rs", set_test_items);
    cases.compile_fail_args("tests/ui/fail/*.rs", set_test_items);
}

#[test]
fn launch_extansion_tests() {
    macrotest::expand("tests/expand/*.rs");
}
