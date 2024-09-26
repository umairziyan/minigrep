use assert_cmd::Command;

#[test]
fn test_help_flag() {
    Command::cargo_bin("minigrep")
        .unwrap()
        .arg("-- help")
        .assert()
        .success()
        .stdout(predicates::str::contains("USAGE"));
}

#[test]
fn test_subcommand_with_args() {
    Command::cargo_bin("minigrep")
        .unwrap()
        .args(&["subcommand", "--option", "value"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Expected Output"));
}

#[test]
fn test_invalid() {
    Command::cargo_bin("minigrep")
        .unwrap()
        .args("--invalid")
        .assert()
        .failure()
        .stdout(predicates::str::contains("update..."));
}
