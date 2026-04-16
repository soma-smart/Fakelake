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
        fs::remove_file("target/csv_deterministic_test.csv").ok();
        fs::remove_file("target/csv_deterministic_test_2.csv").ok();
        fs::remove_file("target/csv_no_seed_test.csv").ok();
        fs::remove_file("target/csv_no_seed_test_2.csv").ok();
        fs::remove_file("target/csv_no_seed_test.yaml").ok();
        fs::remove_file("target/csv_replay_seed_test.csv").ok();
        fs::remove_file("target/csv_replay_seed_test_replayed.csv").ok();
        fs::remove_file("target/csv_replay_seed_test.yaml").ok();
        fs::remove_file("target/csv_replay_seed_test_replayed.yaml").ok();
        fs::remove_file("target/parallel_threads_1.parquet").ok();
        fs::remove_file("target/parallel_threads_8.parquet").ok();
        fs::remove_file("target/parallel_threads.yaml").ok();
        fs::remove_file("target/multifile_seeded.yaml").ok();
        for i in 0..4 {
            fs::remove_file(format!("target/multifile_seeded_run1_{i}.parquet")).ok();
            fs::remove_file(format!("target/multifile_seeded_run2_{i}.parquet")).ok();
        }
        fs::remove_file("target/multifile_naming.yaml").ok();
        for i in 0..3 {
            fs::remove_file(format!("target/multifile_naming_{i}.csv")).ok();
            fs::remove_file(format!("target/multifile_naming_{i}.json")).ok();
        }
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

    #[test]
    fn given_same_seed_should_generate_identical_output() -> Result<(), Box<dyn std::error::Error>>
    {
        let config_path = Path::new("tests/parquet_all_options.yaml");
        let output_path_1 = Path::new("target/parquet_all_options.parquet");
        let output_path_2 = Path::new("target/parquet_all_options_2.parquet");

        // First generation
        let mut cmd1 = Command::cargo_bin("fakelake")?;
        cmd1.arg("generate").arg(config_path).assert().success();

        // Verify first file was created and read its content. Use read (bytes)
        // rather than read_to_string — parquet is a binary format.
        assert!(output_path_1.exists(), "First output file was not created");
        let content1 = fs::read(output_path_1)?;

        // Rename first file to avoid conflict
        fs::rename(output_path_1, output_path_2)?;

        // Second generation
        let mut cmd2 = Command::cargo_bin("fakelake")?;
        cmd2.arg("generate").arg(config_path).assert().success();

        // Verify second file was created and read its content
        assert!(output_path_1.exists(), "Second output file was not created");
        let content2 = fs::read(output_path_1)?;

        // Compare the two files - they should be identical
        assert_eq!(
            content1, content2,
            "Files with same seed should be identical"
        );

        // Clean up
        fs::remove_file(output_path_1).ok();
        fs::remove_file(output_path_2).ok();

        Ok(())
    }

    #[test]
    fn given_no_seed_should_generate_different_output() -> Result<(), Box<dyn std::error::Error>> {
        // Create a test config without seed
        let config_content = r#"
columns:
  - name: id
    provider: Increment.integer
  - name: random_score
    provider: Random.Number.i32
    min: 1
    max: 1000
  - name: random_bool
    provider: Random.bool

info:
  output_name: target/csv_no_seed_test
  output_format: csv
  rows: 50
"#;

        let config_path = Path::new("target/csv_no_seed_test.yaml");
        let output_path_1 = Path::new("target/csv_no_seed_test.csv");
        let output_path_2 = Path::new("target/csv_no_seed_test_2.csv");

        // Create config file
        fs::write(config_path, config_content)?;

        // Clean up any existing files
        fs::remove_file(output_path_1).ok();
        fs::remove_file(output_path_2).ok();

        // First generation
        let mut cmd1 = Command::cargo_bin("fakelake")?;
        cmd1.arg("generate").arg(config_path).assert().success();

        // Verify first file was created and read its content
        assert!(output_path_1.exists(), "First output file was not created");
        let content1 = fs::read_to_string(output_path_1)?;

        // Rename first file to avoid conflict
        fs::rename(output_path_1, output_path_2)?;

        // Second generation
        let mut cmd2 = Command::cargo_bin("fakelake")?;
        cmd2.arg("generate").arg(config_path).assert().success();

        // Verify second file was created and read its content
        assert!(output_path_1.exists(), "Second output file was not created");
        let content2 = fs::read_to_string(output_path_1)?;

        // Compare the two files - they should be different (with high probability)
        // We check that at least some lines are different (excluding the header)
        let lines1: Vec<&str> = content1.lines().collect();
        let lines2: Vec<&str> = content2.lines().collect();

        assert_eq!(
            lines1.len(),
            lines2.len(),
            "Files should have same number of lines"
        );
        assert!(lines1.len() > 1, "Should have more than just header");

        // Header should be identical
        assert_eq!(lines1[0], lines2[0], "Headers should be identical");

        // At least one data line should be different
        let different_lines = lines1
            .iter()
            .zip(lines2.iter())
            .skip(1) // Skip header
            .any(|(line1, line2)| line1 != line2);

        assert!(
            different_lines,
            "At least some data lines should be different when no seed is provided"
        );

        // Clean up
        fs::remove_file(output_path_1).ok();
        fs::remove_file(output_path_2).ok();
        fs::remove_file(config_path).ok();

        Ok(())
    }

    #[test]
    fn given_seeded_parquet_should_be_identical_across_thread_counts(
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Seeded parquet config exercising the parallel writer with presence,
        // corrupted, and variable-length string providers. Rendering it under
        // different RAYON_NUM_THREADS settings must produce byte-identical files.
        // Use a dedicated inline config so this test doesn't race with the
        // shared-fixture parquet tests on target/parquet_all_options.parquet.
        let config_content_1 = r#"
columns:
  - name: id
    provider: Increment.integer
  - name: code
    provider: Random.String.alphanumeric
    length: 5..15
  - name: score
    provider: Random.Number.i32
    min: 0
    max: 1000
    presence: 0.7
    corrupted: 0.05

info:
  output_name: target/parallel_threads_1
  output_format: parquet
  rows: 20000
  seed: 7
"#;
        let config_content_2 =
            config_content_1.replace("target/parallel_threads_1", "target/parallel_threads_8");

        let config_path = Path::new("target/parallel_threads.yaml");
        let out_single = Path::new("target/parallel_threads_1.parquet");
        let out_multi = Path::new("target/parallel_threads_8.parquet");
        fs::remove_file(out_single).ok();
        fs::remove_file(out_multi).ok();

        fs::write(config_path, config_content_1)?;
        Command::cargo_bin("fakelake")?
            .env("RAYON_NUM_THREADS", "1")
            .arg("generate")
            .arg(config_path)
            .assert()
            .success();

        fs::write(config_path, &config_content_2)?;
        Command::cargo_bin("fakelake")?
            .env("RAYON_NUM_THREADS", "8")
            .arg("generate")
            .arg(config_path)
            .assert()
            .success();

        let single_bytes = fs::read(out_single)?;
        let multi_bytes = fs::read(out_multi)?;
        assert_eq!(
            single_bytes, multi_bytes,
            "Seeded parquet output must be identical across thread counts"
        );

        fs::remove_file(out_single).ok();
        fs::remove_file(out_multi).ok();
        fs::remove_file(config_path).ok();
        Ok(())
    }

    #[test]
    fn given_seeded_multifile_parquet_should_be_reproducible(
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 4-file seeded run twice → each file_i is reproducible, and file_0
        // differs from file_1 (different sub-seeds per file-index).
        let config_content = r#"
columns:
  - name: id
    provider: Increment.integer
  - name: code
    provider: Random.String.alphanumeric
    length: 5..15
  - name: optional
    provider: Random.Number.i32
    min: 0
    max: 1000
    presence: 0.7
    corrupted: 0.1

info:
  output_name: target/multifile_seeded_run1
  output_format: parquet
  rows: 500
  files: 4
  seed: 2024
"#;
        let config_path = Path::new("target/multifile_seeded.yaml");
        fs::write(config_path, config_content)?;

        // Run 1
        Command::cargo_bin("fakelake")?
            .arg("generate")
            .arg(config_path)
            .assert()
            .success();
        let run1: Vec<Vec<u8>> = (0..4)
            .map(|i| fs::read(format!("target/multifile_seeded_run1_{i}.parquet")))
            .collect::<Result<_, _>>()?;

        // Run 2 — same config, write to a different prefix so we don't clobber run 1.
        let config_content_2 = config_content.replace(
            "output_name: target/multifile_seeded_run1",
            "output_name: target/multifile_seeded_run2",
        );
        fs::write(config_path, &config_content_2)?;
        Command::cargo_bin("fakelake")?
            .arg("generate")
            .arg(config_path)
            .assert()
            .success();
        let run2: Vec<Vec<u8>> = (0..4)
            .map(|i| fs::read(format!("target/multifile_seeded_run2_{i}.parquet")))
            .collect::<Result<_, _>>()?;

        // Each file_i is byte-identical across the two runs.
        for (i, (a, b)) in run1.iter().zip(run2.iter()).enumerate() {
            assert_eq!(a, b, "file {i} must be byte-identical between seeded runs");
        }

        // And different file indices must differ — sub-seeds are keyed on
        // file-index, so they pick different data.
        assert_ne!(
            run1[0], run1[1],
            "file_0 and file_1 should differ (different sub-seeds)"
        );

        for i in 0..4 {
            fs::remove_file(format!("target/multifile_seeded_run1_{i}.parquet")).ok();
            fs::remove_file(format!("target/multifile_seeded_run2_{i}.parquet")).ok();
        }
        fs::remove_file(config_path).ok();
        Ok(())
    }

    #[test]
    fn given_multifile_csv_and_json_should_name_files_with_index_before_extension(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Path::new("target/multifile_naming.yaml");

        for format in &["csv", "json"] {
            let config_content = format!(
                r#"
columns:
  - name: id
    provider: Increment.integer

info:
  output_name: target/multifile_naming
  output_format: {}
  rows: 3
  files: 3
  seed: 42
"#,
                format
            );
            fs::write(config_path, &config_content)?;
            Command::cargo_bin("fakelake")?
                .arg("generate")
                .arg(config_path)
                .assert()
                .success();

            for i in 0..3 {
                let path = format!("target/multifile_naming_{i}.{format}");
                assert!(Path::new(&path).exists(), "Expected file {} to exist", path);
            }

            // Clean up for next iteration
            for i in 0..3 {
                fs::remove_file(format!("target/multifile_naming_{i}.{format}")).ok();
            }
        }

        fs::remove_file(config_path).ok();
        Ok(())
    }

    #[test]
    fn given_no_seed_printed_seed_should_allow_replay() -> Result<(), Box<dyn std::error::Error>> {
        // Run without a seed and capture the stderr output, which should contain
        // a "seed: <N>" message that lets the user reproduce the run.
        let config_content = r#"
columns:
  - name: id
    provider: Increment.integer
  - name: score
    provider: Random.Number.i32
    min: 0
    max: 10000
  - name: flag
    provider: Random.bool

info:
  output_name: target/csv_replay_seed_test
  output_format: csv
  rows: 100
"#;

        let config_path = Path::new("target/csv_replay_seed_test.yaml");
        let output_path = Path::new("target/csv_replay_seed_test.csv");
        let replayed_path = Path::new("target/csv_replay_seed_test_replayed.csv");
        let replayed_config_path = Path::new("target/csv_replay_seed_test_replayed.yaml");

        fs::write(config_path, config_content)?;
        fs::remove_file(output_path).ok();
        fs::remove_file(replayed_path).ok();

        // First run — no seed. Capture stdout to extract the printed seed.
        let output = Command::cargo_bin("fakelake")?
            .arg("generate")
            .arg(config_path)
            .output()?;
        assert!(output.status.success(), "First run should succeed");

        let stdout = String::from_utf8_lossy(&output.stdout);

        // The seed line looks like: "… seed: 12345678901234 (add …)"
        let seed: u64 = stdout
            .lines()
            .find(|line| line.contains("seed:"))
            .and_then(|line| {
                // Find the first decimal number after "seed: "
                line.split("seed:").nth(1).and_then(|after| {
                    after
                        .split_whitespace()
                        .next()
                        .and_then(|s| s.parse::<u64>().ok())
                })
            })
            .expect("Seed should be printed in stdout when no seed is specified");

        // Second run — replay with the printed seed.
        let replayed_config = format!(
            "{}\n  seed: {}",
            config_content.trim_end().replace(
                "output_name: target/csv_replay_seed_test",
                "output_name: target/csv_replay_seed_test_replayed"
            ),
            seed
        );
        fs::write(replayed_config_path, &replayed_config)?;

        Command::cargo_bin("fakelake")?
            .arg("generate")
            .arg(replayed_config_path)
            .assert()
            .success();

        let original = fs::read_to_string(output_path)?;
        let replayed = fs::read_to_string(replayed_path)?;

        assert_eq!(
            original, replayed,
            "Replaying with the printed seed should produce identical output"
        );

        fs::remove_file(output_path).ok();
        fs::remove_file(replayed_path).ok();
        fs::remove_file(config_path).ok();
        fs::remove_file(replayed_config_path).ok();
        Ok(())
    }
}
