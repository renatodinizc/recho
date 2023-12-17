use assert_cmd::Command;

#[test]
fn with_newline_without_args() {
  let mut cmd = Command::cargo_bin("recho").unwrap();

  cmd.assert().success().stdout("\n");
}

#[test]
fn no_newline_without_args() {
  let mut cmd = Command::cargo_bin("recho").unwrap();

  cmd.arg("-n");

  cmd.assert().success().stdout("");
}

#[test]
fn no_newline_with_args1() {
  let mut cmd = Command::cargo_bin("recho").unwrap();

  cmd.arg("-n");
  cmd.arg("Hello");

  cmd.assert().success().stdout("Hello");
}

#[test]
fn no_newline_with_args2() {
  let mut cmd = Command::cargo_bin("recho").unwrap();

  cmd.arg("-n");
  cmd.arg("Hello World");

  cmd.assert().success().stdout("Hello World");
}

#[test]
fn no_newline_with_args3() {
  let mut cmd = Command::cargo_bin("recho").unwrap();

  cmd.arg("-n");
  cmd.arg("Apple");
  cmd.arg("pie");
  cmd.arg("recipe");

  cmd.assert().success().stdout("Apple pie recipe");
}

#[test]
fn with_newline_with_args1() {
  let mut cmd = Command::cargo_bin("recho").unwrap();

  cmd.arg("Hello");

  cmd.assert().success().stdout("Hello\n");
}

#[test]
fn with_newline_with_args2() {
  let mut cmd = Command::cargo_bin("recho").unwrap();

  cmd.arg("Hello World");

  cmd.assert().success().stdout("Hello World\n");
}

#[test]
fn with_newline_with_args3() {
  let mut cmd = Command::cargo_bin("recho").unwrap();

  cmd.arg("Apple");
  cmd.arg("pie");
  cmd.arg("recipe");

  cmd.assert().success().stdout("Apple pie recipe\n");
}

#[test]
fn with_args_and_option_at_the_middle() {
  let mut cmd = Command::cargo_bin("recho").unwrap();

  cmd.arg("Apple pie");
  cmd.arg("-n");
  cmd.arg("recipe");

  cmd.assert().success().stdout("Apple pie -n recipe\n");
}

#[test]
fn with_args_and_option_at_the_end() {
  let mut cmd = Command::cargo_bin("recho").unwrap();

  cmd.arg("Apple pie recipe");
  cmd.arg("-n");

  cmd.assert().success().stdout("Apple pie recipe -n\n");
}
