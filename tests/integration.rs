use assert_cmd::Command;

#[test]
fn tidy_basic_log() {
    let input = r#"just a message
{"ts":"2024-03-10T12:00:01Z","level":"info","message":"User logged in"}
2024-03-10T12:05:38Z INFO something happened
"#;

    let assert = Command::cargo_bin("logtidier")
        .unwrap()
        .arg("stream")
        .write_stdin(input)
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);

    // Line 1
    assert!(stdout.contains(r#""line":1"#));
    assert!(stdout.contains(r#""message":"just a message""#));
    assert!(stdout.contains(r#""timestamp":null"#));
    assert!(stdout.contains(r#""level":null"#));
    assert!(stdout.contains(r#""fields":null"#));

    // Line 2 (JSON)
    assert!(stdout.contains(r#""line":2"#));
    assert!(stdout.contains(r#""timestamp":"2024-03-10T12:00:01Z""#));
    assert!(stdout.contains(r#""level":"INFO""#));
    assert!(stdout.contains(r#""message":"User logged in""#));

    // Line 3 (timestamp + level + message)
    assert!(stdout.contains(r#""line":3"#));
    assert!(stdout.contains(r#""timestamp":"2024-03-10T12:05:38Z""#));
    assert!(stdout.contains(r#""level":"INFO""#));
    assert!(stdout.contains(r#""message":"something happened""#));
}

#[test]
fn tidy_batch_mode() {
    let input = "2024-03-10T12:05:38Z WARN disk almost full\n";

    let assert = Command::cargo_bin("logtidier")
        .unwrap()
        .arg("batch")
        .write_stdin(input)
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);

    assert!(stdout.contains(r#""line":1"#));
    assert!(stdout.contains(r#""message":"disk almost full""#));
    assert!(stdout.contains(r#""timestamp":"2024-03-10T12:05:38Z""#));
    assert!(stdout.contains(r#""level":"WARN""#));
    assert!(stdout.contains(r#""fields":null"#));
}

#[test]
fn empty_stdin_exits_cleanly() {
    let assert = Command::cargo_bin("logtidier")
        .unwrap()
        .arg("stream")
        .write_stdin("")
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(stdout.trim().is_empty());
}

#[test]
fn blank_lines_skipped_but_increment_line_number() {
    let input = "\n   \n2024-03-10T12:00:01Z INFO after blanks\n";

    let assert = Command::cargo_bin("logtidier")
        .unwrap()
        .arg("stream")
        .write_stdin(input)
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert_eq!(stdout.lines().count(), 1);
    assert!(stdout.contains(r#""line":3"#));
    assert!(stdout.contains(r#""message":"after blanks""#));
}

#[test]
fn malformed_json_and_crlf_handled_gracefully() {
    let input = "{\"ts\":\"2024-03-10T12:00:01Z\", broken\r\n2024-03-10T12:05:38Z INFO windows newline\r\n";

    let assert = Command::cargo_bin("logtidier")
        .unwrap()
        .arg("stream")
        .write_stdin(input)
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(stdout.contains(r#""line":1"#));
    assert!(stdout.contains(r#""line":2"#));
    assert!(stdout.contains(r#""message":"windows newline""#));
    assert!(!stdout.contains(r"\r"));
}

#[test]
fn batch_file_fixture_and_missing_file_error() {
    let assert = Command::cargo_bin("logtidier")
        .unwrap()
        .args(["batch", "--file", "tests/fixtures/sample1.log"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert_eq!(stdout.lines().count(), 15);
    assert!(stdout.contains(r#""fields":{"items":120}"#));

    Command::cargo_bin("logtidier")
        .unwrap()
        .args(["batch", "--file", "tests/fixtures/does-not-exist.log"])
        .assert()
        .failure();
}

#[test]
fn about_strips_escape_codes_when_piped() {
    let assert = Command::cargo_bin("logtidier")
        .unwrap()
        .arg("about")
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(stdout.to_lowercase().contains("logtidier"));
    assert!(stdout.contains("Mohammed Usman"));
    assert!(!stdout.contains("\x1b["));
    assert!(!stdout.contains("\x1b]8;;"));
}

#[test]
fn lt_alias_and_invalid_subcommand() {
    Command::cargo_bin("lt")
        .unwrap()
        .arg("--version")
        .assert()
        .success();

    let assert = Command::cargo_bin("logtidier")
        .unwrap()
        .arg("not-a-command")
        .assert()
        .failure();

    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    assert!(!stderr.is_empty());
}
