#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::fs;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn test_no_args() -> TestResult {
        let mut cmd = Command::cargo_bin("secho")?;
        cmd.assert()
            .failure()
            .stderr("error: the following required arguments were not provided:\n  <TEXT>...\n\nUsage: secho <TEXT>...\n\nFor more information, try '--help'.\n");
        Ok(())
    }

    #[test]
    fn test_run() -> TestResult {
        let mut cmd = Command::cargo_bin("secho")?;
        cmd.arg("hello").assert().success();
        Ok(())
    }

    #[test]
    fn test_hello1() -> TestResult {
        let outfile = "tests/expected/hello1.txt";
        let expected_value = fs::read_to_string(outfile)?;
        let mut cmd = Command::cargo_bin("secho")?;
        cmd.arg("Hello there")
            .assert()
            .success()
            .stdout(expected_value);
        Ok(())
    }

    #[test]
    fn test_hello2() -> TestResult {
        let outfile = "tests/expected/hello2.txt";
        let expected_value = fs::read_to_string(outfile)?;
        let mut cmd = Command::cargo_bin("secho")?;
        cmd.args(vec!["Hello", "there"])
            .assert()
            .success()
            .stdout(expected_value);
        Ok(())
    }

    #[test]
    fn test_hello1_with_new_line() -> TestResult {
        let outfile = "tests/expected/hello1.n.txt";
        let expected_value = fs::read_to_string(outfile)?;
        let mut cmd = Command::cargo_bin("secho")?;
        cmd.args(vec!["Hello there", "-n"])
            .assert()
            .success()
            .stdout(expected_value);
        Ok(())
    }

    #[test]
    fn test_hello2_with_new_line() -> TestResult {
        let outfile = "tests/expected/hello2.n.txt";
        let expected_value = fs::read_to_string(outfile)?;
        let mut cmd = Command::cargo_bin("secho")?;
        cmd.args(vec!["Hello", "there", "-n"])
            .assert()
            .success()
            .stdout(expected_value);
        Ok(())
    }
}
