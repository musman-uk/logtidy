use assert_cmd::Command;

#[test]
fn scaffolding_runs() {
    let mut cmd = Command::cargo_bin("logtidy").unwrap();

    cmd.write_stdin("hello\nworld\n")
        .assert()
        .success()
        .stdout("{\"raw\": \"hello\"}\n{\"raw\": \"world\"}\n");
}
