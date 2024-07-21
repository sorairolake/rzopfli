// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{
    fs::{self, File},
    io::{self, IsTerminal, Read, Write},
};

use anyhow::{bail, Context};
use byte_unit::{Byte, UnitType};
use clap::Parser;
use log::{info, warn};
use simplelog::{ColorChoice, Config, SimpleLogger, TermLogger, TerminalMode};
use zopfli::{Format, Options};

use crate::cli::Opt;

/// Runs the program and returns the result.
#[allow(clippy::too_many_lines)]
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
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .or_else(|_| SimpleLogger::init(log_level, Config::default()))?;

    let zopfli_opt = Options {
        iteration_count: opt.iteration,
        ..Default::default()
    };
    let format = opt.format.into();
    let extension = match format {
        Format::Gzip => ".gz",
        Format::Zlib => ".zlib",
        Format::Deflate => ".deflate",
    };

    for file in opt
        .input
        .map_or_else(|| vec![None], |f| f.into_iter().map(Some).collect())
    {
        let input = match file {
            Some(ref path) if path.as_os_str() != "-" => fs::read(path)
                .with_context(|| format!("could not read data from {}", path.display()))?,
            _ => {
                let mut buf = Vec::new();
                io::stdin()
                    .read_to_end(&mut buf)
                    .context("could not read data from standard input")?;
                buf
            }
        };
        if !opt.quiet {
            if let Some(
                "application/gzip"
                | "application/x-bzip2"
                | "application/x-compress"
                | "application/x-lzip"
                | "application/x-xz"
                | "application/zstd",
            ) = infer::get(&input).map(|t| t.mime_type())
            {
                warn!("input data is already compressed");
            }
        }

        let output_path = file.clone().filter(|p| p.as_os_str() != "-").map(|mut p| {
            p.as_mut_os_string().push(extension);
            p
        });
        if opt.verbose {
            if let Some(ref path) = output_path {
                info!("Saving to: {}", path.display());
            }
        }

        let mut output = Vec::new();
        zopfli::compress(zopfli_opt, format, input.as_slice(), &mut output)
            .context("data could not be compressed")?;

        match file {
            Some(ref path) if path.as_os_str() != "-" && !opt.stdout => {
                let output_path = output_path.context("could not determine the output filename")?;
                if opt.force {
                    File::create(&output_path)
                } else {
                    File::create_new(&output_path)
                }
                .with_context(|| format!("could not open {}", output_path.display()))?
                .write_all(&output)
                .with_context(|| format!("could not write data to {}", output_path.display()))?;
            }
            _ => {
                let mut stdout = io::stdout();
                if stdout.is_terminal() && !(opt.stdout || opt.force) {
                    bail!("compressed data not written to a terminal");
                }
                stdout
                    .write_all(&output)
                    .context("could not write data to stdout")?;
            }
        }

        if opt.remove {
            let input_path = file
                .filter(|p| p.as_os_str() != "-")
                .context("could not get the input filename")?;
            fs::remove_file(&input_path)
                .with_context(|| format!("could not remove {}", input_path.display()))?;
        }

        if opt.verbose {
            let input_size = input.len();
            let output_size = output.len();
            #[allow(clippy::cast_precision_loss)]
            let space_saving = (1.0 - (output_size as f64 / input_size as f64)) * 100.0;
            info!(
                "Original Size: {:#.2}, Compressed: {:#.2}, Compression: {:.2}% Removed",
                Byte::from(input_size).get_appropriate_unit(UnitType::Binary),
                Byte::from(output_size).get_appropriate_unit(UnitType::Binary),
                space_saving
            );
        }
    }
    Ok(())
}
