use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn tidy_basic_log() {
    let input = r#"just a message
{"ts":"2024-03-10T12:00:01Z","level":"info","message":"User logged in"}
2024-03-10T12:05:38Z INFO something happened
"#;

    let mut cmd = Command::cargo_bin("logtidy").unwrap();
    cmd.write_stdin(input)
        .assert()
        .success()
        // Line 1
        .stdout(predicate::str::contains(r#""line":1"#))
        .stdout(predicate::str::contains(r#""message":"just a message""#))
        .stdout(predicate::str::contains(r#""timestamp":null"#))
        .stdout(predicate::str::contains(r#""level":null"#))
        .stdout(predicate::str::contains(r#""fields":null"#))

        // Line 2 (JSON)
        .stdout(predicate::str::contains(r#""line":2"#))
        .stdout(predicate::str::contains(r#""timestamp":"2024-03-10T12:00:01Z""#))
        .stdout(predicate::str::contains(r#""level":"INFO""#))
        .stdout(predicate::str::contains(r#""message":"User logged in""#))

        // Line 3 (timestamp + level + message)
        .stdout(predicate::str::contains(r#""line":3"#))
        .stdout(predicate::str::contains(r#""timestamp":"2024-03-10T12:05:38Z""#))
        .stdout(predicate::str::contains(r#""level":"INFO""#))
        .stdout(predicate::str::contains(r#""message":"something happened""#));
}
