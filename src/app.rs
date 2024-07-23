// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{
    cmp,
    fs::{self, File},
    io::{self, BufReader, IsTerminal, Read, Seek},
};

use anyhow::{bail, Context};
use byte_unit::{Byte, UnitType};
use clap::Parser;
use log::{info, warn};
use simplelog::{ColorChoice, Config, SimpleLogger, TermLogger, TerminalMode};
use zopfli::{Format, Options};

use crate::{cli::Opt, input::Input, output::Output};

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
        TerminalMode::Stderr,
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
            Some(ref path) if path.as_os_str() != "-" => {
                let f = File::open(path)
                    .with_context(|| format!("could not open {}", path.display()))?;
                let size = f
                    .metadata()
                    .map(|m| m.len())
                    .context("could not query metadata about input file")?;
                (Input::File(f), Some(path), Some(size))
            }
            _ => {
                let stdin = io::stdin();
                if stdin.is_terminal() && !opt.force {
                    bail!("standard input is a terminal");
                }
                (Input::Stdin(stdin), None, None)
            }
        };
        if let Input::File(ref f) = input.0 {
            if let Some(size) = input.2.map(usize::try_from).transpose().ok().flatten() {
                let mut f = f;
                let mut file_header = vec![u8::default(); cmp::min(size, 262)];
                f.read_exact(&mut file_header)
                    .context("could not read file header")?;
                f.rewind()
                    .context("could not rewind to beginning of file")?;
                if let Some(
                    "application/gzip"
                    | "application/x-bzip2"
                    | "application/x-compress"
                    | "application/x-lzip"
                    | "application/x-xz"
                    | "application/zstd",
                ) = infer::get(&file_header).map(|t| t.mime_type())
                {
                    warn!("input data is already compressed");
                }
            }
        }

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
            let size = f
                .metadata()
                .map(|m| m.len())
                .context("could not query metadata about output file")?;
            output.2 = Some(size);
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
                fs::remove_file(path)
                    .with_context(|| format!("could not remove {}", path.display()))?;
                info!("{} has been removed", path.display());
            }
        }
    }
    Ok(())
}
