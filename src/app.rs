// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{
    fs::{self, File},
    io::{self, BufReader, IsTerminal},
};

use anyhow::{Context, bail};
use byte_unit::{Byte, UnitType};
use clap::Parser;
use log::{info, warn};
use simplelog::{ColorChoice, Config, SimpleLogger, TermLogger, TerminalMode};
use zopfli::{Format, Options};

use crate::{cli::Opt, input::Input, output::Output};

/// Runs the program and returns the result.
#[allow(clippy::cognitive_complexity, clippy::too_many_lines)]
pub fn run() -> anyhow::Result<()> {
    let opt = Opt::parse();

    if let Some(shell) = opt.generate_completion {
        Opt::print_completion(shell);
        return Ok(());
    }

    let log_level = opt.log_level.into();
    TermLogger::init(
        log_level,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )
    .or_else(|_| SimpleLogger::init(log_level, Config::default()))?;

    let zopfli_opt = Options {
        iteration_count: opt.iteration,
        ..Default::default()
    };
    let format = opt.format.into();
    #[allow(clippy::option_if_let_else)]
    let extension = if let Some(ref suffix) = opt.suffix {
        suffix
    } else {
        match format {
            Format::Gzip => ".gz",
            Format::Zlib => ".zlib",
            Format::Deflate => ".deflate",
        }
    };
    if extension.is_empty() {
        warn!("the suffix is an empty string");
    }

    for file in opt
        .input
        .map_or_else(|| vec![None], |f| f.into_iter().map(Some).collect())
    {
        let input = match file {
            Some(ref path) if path.as_os_str() != "-" => {
                let f = File::open(path)
                    .with_context(|| format!("could not open {}", path.display()))?;
                let size = f.metadata().ok().map(|m| m.len());
                if size.is_none() {
                    warn!("could not query metadata about input file");
                }
                (Input::File(f), Some(path), size)
            }
            _ => {
                let stdin = io::stdin();
                if stdin.is_terminal() && !opt.force {
                    bail!("standard input is a terminal");
                }
                (Input::Stdin(stdin), None, None)
            }
        };

        let mut output = match input.1 {
            Some(path) if !opt.stdout => {
                let mut output_path = path.clone();
                output_path.as_mut_os_string().push(extension);
                let f = if opt.force {
                    File::create(&output_path)
                } else {
                    File::create_new(&output_path)
                }
                .with_context(|| format!("could not open {}", output_path.display()))?;
                (Output::File(f), Some(output_path), None)
            }
            _ => {
                let stdout = io::stdout();
                if stdout.is_terminal() && !(opt.stdout || opt.force) {
                    bail!("compressed data not written to a terminal");
                }
                (Output::Stdout(stdout), None, None)
            }
        };
        if let Some(ref path) = output.1 {
            if !opt.stdout {
                info!("Saving to: {}", path.display());
            }
        }

        zopfli::compress(zopfli_opt, format, BufReader::new(input.0), &mut output.0)
            .context("data could not be compressed")?;

        if let Output::File(f) = output.0 {
            let size = f.metadata().ok().map(|m| m.len());
            if size.is_none() {
                warn!("could not query metadata about output file");
            }
            output.2 = size;
        }
        if let (Some(is), Some(os)) = (input.2, output.2) {
            #[allow(clippy::cast_precision_loss)]
            let space_saving = (1.0 - (os as f64 / is as f64)) * 100.0;
            info!(
                "Original Size: {:#.2}, Compressed: {:#.2}, Compression: {:.2}% Removed",
                Byte::from(is).get_appropriate_unit(UnitType::Binary),
                Byte::from(os).get_appropriate_unit(UnitType::Binary),
                space_saving
            );
        }

        if opt.remove {
            if let Some(path) = input.1 {
                if fs::remove_file(path).is_ok() {
                    info!("{} has been removed", path.display());
                } else {
                    warn!("could not remove {}", path.display());
                }
            }
        }
    }
    Ok(())
}
