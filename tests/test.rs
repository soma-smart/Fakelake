#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    use std::path::Path;

    #[test]
    fn given_no_args_should_fail() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.assert()
           .failure()
           .stderr(predicate::str::contains("Usage: fakelake.exe [OPTIONS] <COMMAND>"));

        Ok(())
    }

    #[test]
    fn given_help_should_succeed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("--help")
           .assert()
           .success()
           .stdout(predicate::str::contains("Usage: fakelake.exe [OPTIONS] <COMMAND>"));

        Ok(())
    }

    #[test]
    fn given_generate_without_file_should_fail() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("generate")
           .assert()
           .failure()
           .stderr(predicate::str::contains("Usage: fakelake.exe generate <PATH_TO_CONFIG>"));

        Ok(())
    }

    #[test]
    fn given_generate_with_one_file_existing_should_succeed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("generate")
           .arg(Path::new("tests/one_row.yaml"))
           .assert()
           .success();

        Ok(())
    }

    #[test]
    fn given_generate_with_one_file_not_existing_should_fail() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("generate")
           .arg(Path::new("this/is/not_a_file.yaml"))
           .assert()
           .failure();

        Ok(())
    }

    #[test]
    fn given_generate_with_one_file_not_yaml_should_fail() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("generate")
           .arg(Path::new("src/main.rs"))
           .assert()
           .failure();

        Ok(())
    }

    #[test]
    fn given_generate_with_multiple_file_existing_should_succeed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("generate")
           .arg(Path::new("tests/one_row.yaml"))
           .arg(Path::new("tests/one_row.yaml"))
           .assert()
           .success();

        Ok(())
    }

    #[test]
    fn given_generate_with_multiple_file_with_one_not_existing_should_fail() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("generate")
           .arg(Path::new("tests/one_row.yaml"))
           .arg(Path::new("this/is/not_a_file.yaml"))
           .assert()
           .failure();

        Ok(())
    }

    #[test]
    fn given_generate_one_file_with_verbose_should_succeed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("-v")
           .arg("generate")
           .arg(Path::new("tests/one_row.yaml"))
           .assert()
           .success();

        Ok(())
    }
}
