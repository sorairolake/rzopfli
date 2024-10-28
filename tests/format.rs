// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod utils;

use std::{fs, io::Read};

use flate2::read::{DeflateDecoder, GzDecoder, ZlibDecoder};
use predicates::prelude::predicate;

const TEST_DATA: &[u8] = include_bytes!("data/LICENSES/CC-BY-4.0.txt");

#[test]
fn compress_to_gzip() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    let mut output_filename = input_filename.clone();
    output_filename.as_mut_os_string().push(".gz");
    assert!(!output_filename.exists());
    utils::command::command()
        .arg("--format")
        .arg("gzip")
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

#[test]
fn compress_to_zlib() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    let mut output_filename = input_filename.clone();
    output_filename.as_mut_os_string().push(".zlib");
    assert!(!output_filename.exists());
    utils::command::command()
        .arg("--format")
        .arg("zlib")
        .arg(input_filename)
        .assert()
        .success();
    let compressed_data = fs::read(output_filename).unwrap();
    assert_ne!(compressed_data, TEST_DATA);
    assert!(compressed_data.len() < TEST_DATA.len());
    let mut decoder = ZlibDecoder::new(compressed_data.as_slice());
    let mut buf = [u8::default(); TEST_DATA.len()];
    decoder.read_exact(&mut buf).unwrap();
    assert_eq!(buf, TEST_DATA);
}

#[test]
fn compress_to_deflate() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    let mut output_filename = input_filename.clone();
    output_filename.as_mut_os_string().push(".deflate");
    assert!(!output_filename.exists());
    utils::command::command()
        .arg("--format")
        .arg("deflate")
        .arg(input_filename)
        .assert()
        .success();
    let compressed_data = fs::read(output_filename).unwrap();
    assert_ne!(compressed_data, TEST_DATA);
    assert!(compressed_data.len() < TEST_DATA.len());
    let mut decoder = DeflateDecoder::new(compressed_data.as_slice());
    let mut buf = [u8::default(); TEST_DATA.len()];
    decoder.read_exact(&mut buf).unwrap();
    assert_eq!(buf, TEST_DATA);
}

#[test]
fn compress_to_default_format() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    let mut output_filename = input_filename.clone();
    output_filename.as_mut_os_string().push(".gz");
    assert!(!output_filename.exists());
    utils::command::command()
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

#[test]
fn compress_to_invalid_format() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let input_filename = temp_dir_path.join("foo.txt");
    fs::write(&input_filename, TEST_DATA).unwrap();
    utils::command::command()
        .arg("--format")
        .arg("zstd")
        .arg(input_filename)
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'zstd' for '--format <FORMAT>'",
        ));
}
