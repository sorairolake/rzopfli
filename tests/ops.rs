// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]
#![allow(clippy::multiple_crate_versions)]

mod utils;

use std::{
    fs::{self, File},
    io::Read,
};

use flate2::read::GzDecoder;
use predicates::prelude::predicate;

const TEST_DATA: &[u8] = include_bytes!("data/LICENSES/CC-BY-4.0.txt");

#[test]
fn compress_from_stdin() {
    {
        let output = utils::command::command()
            .arg("-c")
            .write_stdin(TEST_DATA)
            .output()
            .unwrap();
        let compressed_data = output.stdout;
        assert_ne!(compressed_data, TEST_DATA);
        assert!(compressed_data.len() < TEST_DATA.len());
        let mut decoder = GzDecoder::new(compressed_data.as_slice());
        let mut buf = [u8::default(); TEST_DATA.len()];
        decoder.read_exact(&mut buf).unwrap();
        assert_eq!(buf, TEST_DATA);
        assert!(output.status.success());
    }
    {
        let output = utils::command::command()
            .arg("-f")
            .write_stdin(TEST_DATA)
            .output()
            .unwrap();
        let compressed_data = output.stdout;
        assert_ne!(compressed_data, TEST_DATA);
        assert!(compressed_data.len() < TEST_DATA.len());
        let mut decoder = GzDecoder::new(compressed_data.as_slice());
        let mut buf = [u8::default(); TEST_DATA.len()];
        decoder.read_exact(&mut buf).unwrap();
        assert_eq!(buf, TEST_DATA);
        assert!(output.status.success());
    }
    {
        let output = utils::command::command()
            .arg("-c")
            .arg("-")
            .write_stdin(TEST_DATA)
            .output()
            .unwrap();
        let compressed_data = output.stdout;
        assert_ne!(compressed_data, TEST_DATA);
        assert!(compressed_data.len() < TEST_DATA.len());
        let mut decoder = GzDecoder::new(compressed_data.as_slice());
        let mut buf = [u8::default(); TEST_DATA.len()];
        decoder.read_exact(&mut buf).unwrap();
        assert_eq!(buf, TEST_DATA);
        assert!(output.status.success());
    }
    {
        let output = utils::command::command()
            .arg("-f")
            .arg("-")
            .write_stdin(TEST_DATA)
            .output()
            .unwrap();
        let compressed_data = output.stdout;
        assert_ne!(compressed_data, TEST_DATA);
        assert!(compressed_data.len() < TEST_DATA.len());
        let mut decoder = GzDecoder::new(compressed_data.as_slice());
        let mut buf = [u8::default(); TEST_DATA.len()];
        decoder.read_exact(&mut buf).unwrap();
        assert_eq!(buf, TEST_DATA);
        assert!(output.status.success());
    }
}

#[test]
fn write_to_stdout() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    let output = utils::command::command()
        .arg("-c")
        .arg(input_filename)
        .output()
        .unwrap();
    let compressed_data = output.stdout;
    assert_ne!(compressed_data, TEST_DATA);
    assert!(compressed_data.len() < TEST_DATA.len());
    let mut decoder = GzDecoder::new(compressed_data.as_slice());
    let mut buf = [u8::default(); TEST_DATA.len()];
    decoder.read_exact(&mut buf).unwrap();
    assert_eq!(buf, TEST_DATA);
    assert!(output.status.success());
}

#[test]
fn write_to_stdout_conflicts_with_remove() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    utils::command::command()
        .arg("-c")
        .arg("--rm")
        .arg(input_filename)
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--stdout' cannot be used with '--rm'",
        ));
}

#[test]
fn write_to_stdout_conflicts_with_suffix() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    let mut output_filename = input_filename.clone();
    output_filename.as_mut_os_string().push(".gzip");
    assert!(!output_filename.exists());
    utils::command::command()
        .arg("-c")
        .arg("-S")
        .arg(".gzip")
        .arg(input_filename)
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--stdout' cannot be used with '--suffix <SUFFIX>'",
        ));
}

#[test]
fn compress_with_force() {
    {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_path = temp_dir.path();
        let input_filename = temp_dir_path.join("foo.txt");
        fs::write(&input_filename, TEST_DATA).unwrap();
        let mut output_filename = input_filename.clone();
        output_filename.as_mut_os_string().push(".gz");
        File::create_new(&output_filename).unwrap();
        assert!(output_filename.exists());
        let command = utils::command::command()
            .arg(input_filename)
            .assert()
            .failure()
            .code(73)
            .stderr(predicate::str::contains(format!(
                "could not open {}",
                output_filename.display()
            )));
        if cfg!(windows) {
            command.stderr(predicate::str::contains("The file exists. (os error 80)"));
        } else {
            command.stderr(predicate::str::contains("File exists (os error 17)"));
        }
    }
    {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_path = temp_dir.path();
        let input_filename = temp_dir_path.join("foo.txt");
        fs::write(&input_filename, TEST_DATA).unwrap();
        let mut output_filename = input_filename.clone();
        output_filename.as_mut_os_string().push(".gz");
        File::create_new(&output_filename).unwrap();
        assert!(output_filename.exists());
        utils::command::command()
            .arg("-f")
            .arg(input_filename)
            .assert()
            .success();
        let compressed_data = fs::read(output_filename).unwrap();
        assert_ne!(compressed_data, TEST_DATA);
        assert!(compressed_data.len() < TEST_DATA.len());
        let mut decoder = GzDecoder::new(compressed_data.as_slice());
        let mut buf = [u8::default(); TEST_DATA.len()];
        decoder.read_exact(&mut buf).unwrap();
        assert_eq!(buf, TEST_DATA);
    }
}

#[test]
fn compress_with_keep() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    let mut output_filename = input_filename.clone();
    output_filename.as_mut_os_string().push(".gz");
    assert!(!output_filename.exists());
    utils::command::command()
        .arg("-k")
        .arg(&input_filename)
        .assert()
        .success();
    let compressed_data = fs::read(output_filename).unwrap();
    assert_ne!(compressed_data, TEST_DATA);
    assert!(compressed_data.len() < TEST_DATA.len());
    let mut decoder = GzDecoder::new(compressed_data.as_slice());
    let mut buf = [u8::default(); TEST_DATA.len()];
    decoder.read_exact(&mut buf).unwrap();
    assert_eq!(buf, TEST_DATA);
    assert!(input_filename.exists());
}

#[test]
fn compress_with_remove() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    let mut output_filename = input_filename.clone();
    output_filename.as_mut_os_string().push(".gz");
    assert!(!output_filename.exists());
    utils::command::command()
        .arg("--rm")
        .arg(&input_filename)
        .assert()
        .success()
        .stderr(predicate::str::contains(format!(
            "{} has been removed",
            input_filename.display()
        )));
    let compressed_data = fs::read(output_filename).unwrap();
    assert_ne!(compressed_data, TEST_DATA);
    assert!(compressed_data.len() < TEST_DATA.len());
    let mut decoder = GzDecoder::new(compressed_data.as_slice());
    let mut buf = [u8::default(); TEST_DATA.len()];
    decoder.read_exact(&mut buf).unwrap();
    assert_eq!(buf, TEST_DATA);
    assert!(!input_filename.exists());
}

#[test]
fn compress_with_keep_conflicts_with_remove() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    let mut output_filename = input_filename.clone();
    output_filename.as_mut_os_string().push(".gz");
    assert!(!output_filename.exists());
    utils::command::command()
        .arg("-k")
        .arg("--rm")
        .arg(input_filename)
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--keep' cannot be used with '--rm'",
        ));
}

#[test]
fn compress_with_suffix() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    let mut output_filename = input_filename.clone();
    output_filename.as_mut_os_string().push(".gzip");
    assert!(!output_filename.exists());
    utils::command::command()
        .arg("-S")
        .arg(".gzip")
        .arg(&input_filename)
        .assert()
        .success();
    let compressed_data = fs::read(output_filename).unwrap();
    assert_ne!(compressed_data, TEST_DATA);
    assert!(compressed_data.len() < TEST_DATA.len());
    let mut decoder = GzDecoder::new(compressed_data.as_slice());
    let mut buf = [u8::default(); TEST_DATA.len()];
    decoder.read_exact(&mut buf).unwrap();
    assert_eq!(buf, TEST_DATA);
    assert!(input_filename.exists());
}

#[test]
fn compress_with_empty_suffix() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    let output_filename = &input_filename;
    assert!(output_filename.exists());
    let command = utils::command::command()
        .arg("-S")
        .arg("")
        .arg(&input_filename)
        .assert()
        .failure()
        .code(73)
        .stderr(predicate::str::contains("the suffix is an empty string"))
        .stderr(predicate::str::contains(format!(
            "could not open {}",
            output_filename.display()
        )));
    if cfg!(windows) {
        command.stderr(predicate::str::contains("The file exists. (os error 80)"));
    } else {
        command.stderr(predicate::str::contains("File exists (os error 17)"));
    }
}
