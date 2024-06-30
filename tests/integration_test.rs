use assert_cmd::Command;

#[test]
fn test_lcov_parsing_and_output() {
    let lcov_file_path = "./tests/fixtures/lcov.info";

    let assert = Command::cargo_bin("lcov_to_checkstyle")
        .unwrap()
        .arg(lcov_file_path)
        .assert()
        .success();

    let output = assert.get_output().stdout.clone();
    let output_str = String::from_utf8(output).unwrap();

    let expected_output = r#"
<checkstyle version="4.3">
    <file name="/path/to/project/src/main.rs">
        <error line="4" severity="warning" message="Lines 3-4 are not covered" source="coverage"/>
        <error line="7" severity="warning" message="Line 7 is not covered" source="coverage"/>
    </file>
    <file name="/path/to/project/src/lib.rs">
        <error line="3" severity="warning" message="Lines 2-3 are not covered" source="coverage"/>
        <error line="6" severity="warning" message="Line 6 is not covered" source="coverage"/>
    </file>
</checkstyle>
"#;

    assert_eq!(
        output_str.replace(" ", "").replace("\n", ""),
        expected_output.replace(" ", "").replace("\n", "")
    );
}
