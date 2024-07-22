// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{fs::File, io::Stdout};

#[derive(Debug)]
pub enum Output {
    File(File),
    Stdout(Stdout),
}
