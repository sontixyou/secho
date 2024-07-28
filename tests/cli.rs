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
}
