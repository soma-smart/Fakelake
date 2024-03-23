#[cfg(windows)]
const FAKELAKE_COMMAND_NAME: &str = "fakelake.exe";
#[cfg(not(windows))]
const FAKELAKE_COMMAND_NAME: &str = "fakelake";

#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use ctor::{ctor, dtor};
    use predicates::prelude::*;
    use std::fs;
    use std::process::Command;

    use std::path::Path;

    use crate::FAKELAKE_COMMAND_NAME;

    #[ctor]
    fn init() {
        fs::create_dir_all("target/test_generated").ok();
    }

    #[dtor]
    fn shutdown() {
        fs::remove_dir_all("target/test_generated").ok();
        fs::remove_file("output.csv").ok();
        fs::remove_file("output.json").ok();
    }

    #[test]
    fn given_no_args_should_fail() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.assert()
            .failure()
            .stderr(predicate::str::contains(format!(
                "Usage: {} [OPTIONS] <COMMAND>",
                FAKELAKE_COMMAND_NAME
            )));

        Ok(())
    }

    #[test]
    fn given_help_should_succeed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Usage: {} [OPTIONS] <COMMAND>",
                FAKELAKE_COMMAND_NAME
            )));

        Ok(())
    }

    #[test]
    fn given_generate_without_file_should_fail() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("generate")
            .assert()
            .failure()
            .stderr(predicate::str::contains(format!(
                "Usage: {} generate <PATH_TO_CONFIG>",
                FAKELAKE_COMMAND_NAME
            )));

        Ok(())
    }

    #[test]
    fn given_generate_with_one_file_existing_should_succeed(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("generate")
            .arg(Path::new("tests/one_row_parquet.yaml"))
            .assert()
            .success();

        Ok(())
    }

    #[test]
    fn given_generate_with_one_file_not_existing_should_fail(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("generate")
            .arg(Path::new("this/is/not_a_file.yaml"))
            .assert()
            .failure();

        Ok(())
    }

    #[test]
    fn given_generate_with_one_file_not_yaml_should_fail() -> Result<(), Box<dyn std::error::Error>>
    {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("generate")
            .arg(Path::new("src/main.rs"))
            .assert()
            .failure();

        Ok(())
    }

    #[test]
    fn given_generate_with_multiple_file_existing_should_succeed(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("generate")
            .arg(Path::new("tests/one_row_parquet.yaml"))
            .arg(Path::new("tests/one_row_parquet.yaml"))
            .assert()
            .success();

        Ok(())
    }

    #[test]
    fn given_generate_with_multiple_file_with_one_not_existing_should_fail(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("generate")
            .arg(Path::new("tests/one_row_parquet.yaml"))
            .arg(Path::new("this/is/not_a_file.yaml"))
            .assert()
            .failure();

        Ok(())
    }

    #[test]
    fn given_generate_one_parquet_file_with_verbose_should_succeed(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("-v")
            .arg("generate")
            .arg(Path::new("tests/one_row_parquet.yaml"))
            .assert()
            .success();

        Ok(())
    }

    #[test]
    fn given_generate_one_csv_file_with_verbose_should_succeed(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("-v")
            .arg("generate")
            .arg(Path::new("tests/one_row_csv.yaml"))
            .assert()
            .success();

        Ok(())
    }

    #[test]
    fn given_generate_one_json_file_with_verbose_should_succeed(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("fakelake")?;

        cmd.arg("-v")
            .arg("generate")
            .arg(Path::new("tests/one_row_json.yaml"))
            .assert()
            .success();

        Ok(())
    }
}
