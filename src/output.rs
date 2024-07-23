// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{
    fs::File,
    io::{self, Stdout, Write},
};

#[derive(Debug)]
pub enum Output {
    File(File),
    Stdout(Stdout),
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            Self::File(ref mut file) => file.write(buf),
            Self::Stdout(ref mut stdout) => stdout.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            Self::File(ref mut file) => file.flush(),
            Self::Stdout(ref mut stdout) => stdout.flush(),
        }
    }
}
