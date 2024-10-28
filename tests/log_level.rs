// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod utils;

use std::fs;

use predicates::prelude::predicate;

const TEST_DATA: &[u8] = include_bytes!("data/LICENSES/CC-BY-4.0.txt");

#[test]
fn compress_with_off_log_level() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    let mut output_filename = input_filename.clone();
    output_filename.as_mut_os_string().push(".gz");
    assert!(!output_filename.exists());
    utils::command::command()
        .arg("--log-level")
        .arg("OFF")
        .arg(input_filename)
        .assert()
        .success()
        .stderr(predicate::str::is_empty());
}

#[test]
fn compress_with_error_log_level() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    let mut output_filename = input_filename.clone();
    output_filename.as_mut_os_string().push(".gz");
    assert!(!output_filename.exists());
    utils::command::command()
        .arg("--log-level")
        .arg("ERROR")
        .arg(input_filename)
        .assert()
        .success()
        .stderr(predicate::str::is_empty());
}

#[test]
fn compress_with_warn_log_level() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    let mut output_filename = input_filename.clone();
    output_filename.as_mut_os_string().push(".gz");
    assert!(!output_filename.exists());
    utils::command::command()
        .arg("--log-level")
        .arg("WARN")
        .arg(input_filename)
        .assert()
        .success()
        .stderr(predicate::str::is_empty());
}

#[test]
fn compress_with_info_log_level() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    let mut output_filename = input_filename.clone();
    output_filename.as_mut_os_string().push(".gz");
    assert!(!output_filename.exists());
    utils::command::command()
        .arg("--log-level")
        .arg("INFO")
        .arg(input_filename)
        .assert()
        .success()
        .stderr(predicate::str::contains(format!(
            "Saving to: {}",
            output_filename.display()
        )))
        .stderr(predicate::str::contains(
            "Original Size: 18.22 KiB, Compressed: 5.47 KiB, Compression: 69.96% Removed",
        ));
}

#[test]
fn compress_with_debug_log_level() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    let mut output_filename = input_filename.clone();
    output_filename.as_mut_os_string().push(".gz");
    assert!(!output_filename.exists());
    utils::command::command()
        .arg("--log-level")
        .arg("DEBUG")
        .arg(input_filename)
        .assert()
        .success()
        .stderr(predicate::str::contains(format!(
            "Saving to: {}",
            output_filename.display()
        )))
        .stderr(predicate::str::contains("Iteration"))
        .stderr(predicate::str::contains(
            "Original Size: 18.22 KiB, Compressed: 5.47 KiB, Compression: 69.96% Removed",
        ));
}

#[test]
fn compress_with_trace_log_level() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    let mut output_filename = input_filename.clone();
    output_filename.as_mut_os_string().push(".gz");
    assert!(!output_filename.exists());
    utils::command::command()
        .arg("--log-level")
        .arg("TRACE")
        .arg(input_filename)
        .assert()
        .success()
        .stderr(predicate::str::contains(format!(
            "Saving to: {}",
            output_filename.display()
        )))
        .stderr(predicate::str::contains("Iteration"))
        .stderr(predicate::str::contains(
            "Original Size: 18.22 KiB, Compressed: 5.47 KiB, Compression: 69.96% Removed",
        ));
}

#[test]
fn compress_with_invalid_log_level() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    utils::command::command()
        .arg("--log-level")
        .arg("a")
        .arg(input_filename)
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'a' for '--log-level <LEVEL>'",
        ));
}
