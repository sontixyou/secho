#[cfg(test)]
mod tests {
    use assert_cmd;
    use predicates::prelude::*;
    use std::fs;

    #[test]
    fn test_no_args() {
        let mut cmd = assert_cmd::Command::cargo_bin("secho").unwrap();
        cmd.assert()
            .failure()
            .stderr("error: the following required arguments were not provided:\n  <TEXT>...\n\nUsage: secho <TEXT>...\n\nFor more information, try '--help'.\n");
    }

    #[test]
    fn test_run() {
        let mut cmd = assert_cmd::Command::cargo_bin("secho").unwrap();
        cmd.arg("hello").assert().success();
    }

    #[test]
    fn test_hello1() {
        let outfile = "tests/expected/hello1.txt";
        let expected_value = fs::read_to_string(outfile).unwrap();
        let mut cmd = assert_cmd::Command::cargo_bin("secho").unwrap();
        cmd.arg("Hello there")
            .assert()
            .success()
            .stdout(expected_value);
    }
}
