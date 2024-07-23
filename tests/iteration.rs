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

use std::{fs, io::Read};

use flate2::read::GzDecoder;
use predicates::prelude::predicate;

const TEST_DATA: &[u8] = include_bytes!("data/LICENSES/MIT.txt");

#[test]
fn compress_with_1_iteration() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    let mut output_filename = input_filename.clone();
    output_filename.as_mut_os_string().push(".gz");
    assert!(!output_filename.exists());
    utils::command::command()
        .arg("-i")
        .arg("1")
        .arg(input_filename)
        .assert()
        .success()
        .stderr(predicate::str::contains(format!(
            "Saving to: {}",
            output_filename.display()
        )))
        .stderr(predicate::str::contains(
            "Original Size: 1.04 KiB, Compressed: 618 B, Compression: 42.08% Removed",
        ));
    let compressed_data = fs::read(output_filename).unwrap();
    assert_ne!(compressed_data, TEST_DATA);
    assert!(compressed_data.len() < TEST_DATA.len());
    let mut decoder = GzDecoder::new(compressed_data.as_slice());
    let mut buf = [u8::default(); TEST_DATA.len()];
    decoder.read_exact(&mut buf).unwrap();
    assert_eq!(buf, TEST_DATA);
}

#[test]
fn compress_with_10_iterations() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    let mut output_filename = input_filename.clone();
    output_filename.as_mut_os_string().push(".gz");
    assert!(!output_filename.exists());
    utils::command::command()
        .arg("-i")
        .arg("10")
        .arg(input_filename)
        .assert()
        .success()
        .stderr(predicate::str::contains(format!(
            "Saving to: {}",
            output_filename.display()
        )))
        .stderr(predicate::str::contains(
            "Original Size: 1.04 KiB, Compressed: 616 B, Compression: 42.27% Removed",
        ));
    let compressed_data = fs::read(output_filename).unwrap();
    assert_ne!(compressed_data, TEST_DATA);
    assert!(compressed_data.len() < TEST_DATA.len());
    let mut decoder = GzDecoder::new(compressed_data.as_slice());
    let mut buf = [u8::default(); TEST_DATA.len()];
    decoder.read_exact(&mut buf).unwrap();
    assert_eq!(buf, TEST_DATA);
}

#[test]
fn compress_with_50_iterations() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    let mut output_filename = input_filename.clone();
    output_filename.as_mut_os_string().push(".gz");
    assert!(!output_filename.exists());
    utils::command::command()
        .arg("-i")
        .arg("50")
        .arg(input_filename)
        .assert()
        .success()
        .stderr(predicate::str::contains(format!(
            "Saving to: {}",
            output_filename.display()
        )))
        .stderr(predicate::str::contains(
            "Original Size: 1.04 KiB, Compressed: 613 B, Compression: 42.55% Removed",
        ));
    let compressed_data = fs::read(output_filename).unwrap();
    assert_ne!(compressed_data, TEST_DATA);
    assert!(compressed_data.len() < TEST_DATA.len());
    let mut decoder = GzDecoder::new(compressed_data.as_slice());
    let mut buf = [u8::default(); TEST_DATA.len()];
    decoder.read_exact(&mut buf).unwrap();
    assert_eq!(buf, TEST_DATA);
}

#[test]
fn compress_with_1000_iterations() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    let mut output_filename = input_filename.clone();
    output_filename.as_mut_os_string().push(".gz");
    assert!(!output_filename.exists());
    utils::command::command()
        .arg("-i")
        .arg("1000")
        .arg(input_filename)
        .assert()
        .success()
        .stderr(predicate::str::contains(format!(
            "Saving to: {}",
            output_filename.display()
        )))
        .stderr(predicate::str::contains(
            "Original Size: 1.04 KiB, Compressed: 612 B, Compression: 42.64% Removed",
        ));
    let compressed_data = fs::read(output_filename).unwrap();
    assert_ne!(compressed_data, TEST_DATA);
    assert!(compressed_data.len() < TEST_DATA.len());
    let mut decoder = GzDecoder::new(compressed_data.as_slice());
    let mut buf = [u8::default(); TEST_DATA.len()];
    decoder.read_exact(&mut buf).unwrap();
    assert_eq!(buf, TEST_DATA);
}

#[test]
fn compress_with_invalid_iterations() {
    {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_path = temp_dir.path();
        let input_filename = temp_dir_path.join("foo.txt");
        fs::write(&input_filename, TEST_DATA).unwrap();
        utils::command::command()
            .arg("-i")
            .arg("0")
            .arg(input_filename)
            .assert()
            .failure()
            .code(2)
            .stderr(predicate::str::contains(
                "invalid value '0' for '--iteration <TIMES>'",
            ))
            .stderr(predicate::str::contains(
                "number would be zero for non-zero type",
            ));
    }
    {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_path = temp_dir.path();
        let input_filename = temp_dir_path.join("foo.txt");
        fs::write(&input_filename, TEST_DATA).unwrap();
        utils::command::command()
            .arg("-i")
            .arg("a")
            .arg(input_filename)
            .assert()
            .failure()
            .code(2)
            .stderr(predicate::str::contains(
                "invalid value 'a' for '--iteration <TIMES>'",
            ))
            .stderr(predicate::str::contains("invalid digit found in string"));
    }
}
