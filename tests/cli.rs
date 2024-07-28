#[cfg(test)]
mod tests {
    use assert_cmd;

    #[test]
    fn test_no_args() {
        let mut cmd = assert_cmd::Command::cargo_bin("secho").unwrap();
        cmd.assert().failure();
    }
}
