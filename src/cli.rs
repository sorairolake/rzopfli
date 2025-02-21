// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{
    io::{self, Write},
    num::NonZeroU64,
    ops::Deref,
    path::{self, PathBuf},
    str::FromStr,
};

use anyhow::bail;
use clap::{CommandFactory, Parser, ValueEnum, ValueHint};
use clap_complete::Generator;
use simplelog::LevelFilter;

const LONG_VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    '\n',
    "Copyright (C) 2024 Shun Sakai\n",
    '\n',
    "This program is distributed under the terms of either the Apache License 2.0 or\n",
    "the MIT License.\n",
    '\n',
    "This is free software: you are free to change and redistribute it. There is NO\n",
    "WARRANTY, to the extent permitted by law.\n",
    '\n',
    "Report bugs to <https://github.com/sorairolake/rzopfli/issues>."
);

const AFTER_LONG_HELP: &str = "See `rzopfli(1)` for more details.";

#[derive(Debug, Parser)]
#[allow(clippy::struct_excessive_bools)]
#[command(
    version,
    long_version(LONG_VERSION),
    about,
    max_term_width(100),
    after_long_help(AFTER_LONG_HELP)
)]
pub struct Opt {
    /// Write to standard output, keep original files.
    #[arg(short('c'), long, conflicts_with("remove"), conflicts_with("suffix"))]
    pub stdout: bool,

    /// Force compression even if the output file already exists.
    ///
    /// This option allows you to overwrite existing files.
    #[arg(short, long)]
    pub force: bool,

    /// Keep input files.
    ///
    /// This is the default behavior.
    #[arg(short, long, conflicts_with("remove"))]
    pub _keep: bool,

    /// Remove input files after successful compression.
    #[arg(long("rm"))]
    pub remove: bool,

    /// Use <SUFFIX> as the suffix for the target file instead of '.gz',
    /// '.zlib', or '.deflate'.
    ///
    /// Any non-empty UTF-8 string which starts with '.' and does not contains a
    /// path separator can be specified as the suffix.
    #[arg(short('S'), long, value_name("SUFFIX"))]
    pub suffix: Option<Suffix>,

    /// Perform compression for the specified number of iterations.
    ///
    /// Higher numbers produce higher compression ratio at the expense of
    /// compression speed.
    #[arg(short, long, default_value("15"), value_name("TIMES"))]
    pub iteration: NonZeroU64,

    /// Output to the specified format.
    #[arg(
        long,
        value_enum,
        default_value_t,
        value_name("FORMAT"),
        ignore_case(true)
    )]
    pub format: Format,

    /// The minimum log level to print.
    #[arg(
        long,
        value_enum,
        default_value_t,
        value_name("LEVEL"),
        ignore_case(true)
    )]
    pub log_level: LogLevel,

    /// Generate shell completion.
    ///
    /// The completion is output to standard output.
    #[arg(long, value_enum, value_name("SHELL"))]
    pub generate_completion: Option<Shell>,

    /// Files to compress.
    ///
    /// If [FILE] is not specified, or if "-" is specified, data will be read
    /// from standard input.
    #[arg(value_name("FILE"), value_hint(ValueHint::FilePath))]
    pub input: Option<Vec<PathBuf>>,
}

impl Opt {
    /// Generates shell completion and print it.
    pub fn print_completion(generator: impl Generator) {
        clap_complete::generate(
            generator,
            &mut Self::command(),
            Self::command().get_name(),
            &mut io::stdout(),
        );
    }
}

#[derive(Clone, Debug, ValueEnum)]
#[allow(clippy::doc_markdown)]
#[value(rename_all = "lower")]
pub enum Shell {
    /// Bash.
    Bash,

    /// Elvish.
    Elvish,

    /// fish.
    Fish,

    /// Nushell.
    Nushell,

    #[allow(clippy::enum_variant_names)]
    /// PowerShell.
    PowerShell,

    /// Zsh.
    Zsh,
}

impl Generator for Shell {
    fn file_name(&self, name: &str) -> String {
        match self {
            Self::Bash => clap_complete::Shell::Bash.file_name(name),
            Self::Elvish => clap_complete::Shell::Elvish.file_name(name),
            Self::Fish => clap_complete::Shell::Fish.file_name(name),
            Self::Nushell => clap_complete_nushell::Nushell.file_name(name),
            Self::PowerShell => clap_complete::Shell::PowerShell.file_name(name),
            Self::Zsh => clap_complete::Shell::Zsh.file_name(name),
        }
    }

    fn generate(&self, cmd: &clap::Command, buf: &mut dyn Write) {
        match self {
            Self::Bash => clap_complete::Shell::Bash.generate(cmd, buf),
            Self::Elvish => clap_complete::Shell::Elvish.generate(cmd, buf),
            Self::Fish => clap_complete::Shell::Fish.generate(cmd, buf),
            Self::Nushell => clap_complete_nushell::Nushell.generate(cmd, buf),
            Self::PowerShell => clap_complete::Shell::PowerShell.generate(cmd, buf),
            Self::Zsh => clap_complete::Shell::Zsh.generate(cmd, buf),
        }
    }
}

/// The suffix for the target file.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Suffix(String);

impl Deref for Suffix {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Suffix {
    type Err = anyhow::Error;

    fn from_str(suffix: &str) -> anyhow::Result<Self> {
        if suffix.is_empty() {
            bail!("the suffix is an empty string");
        }
        if suffix.contains(path::MAIN_SEPARATOR) {
            bail!("the suffix contains a path separator");
        }
        if !suffix.starts_with('.') {
            bail!("the suffix does not starts with `.`");
        }
        Ok(Self(suffix.into()))
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, ValueEnum)]
pub enum Format {
    /// The gzip file format, as defined in RFC 1952.
    #[default]
    Gzip,

    /// The zlib file format, as defined in RFC 1950.
    Zlib,

    /// The raw DEFLATE stream format, as defined in RFC 1951.
    Deflate,
}

impl From<Format> for zopfli::Format {
    fn from(format: Format) -> Self {
        match format {
            Format::Gzip => Self::Gzip,
            Format::Zlib => Self::Zlib,
            Format::Deflate => Self::Deflate,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, ValueEnum)]
#[value(rename_all = "UPPER")]
pub enum LogLevel {
    /// Lowest log level.
    Off,

    /// Error log level.
    Error,

    /// Warn log level.
    Warn,

    /// Info log level.
    #[default]
    Info,

    /// Debug log level.
    Debug,

    /// Trace log level.
    Trace,
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Off => Self::Off,
            LogLevel::Error => Self::Error,
            LogLevel::Warn => Self::Warn,
            LogLevel::Info => Self::Info,
            LogLevel::Debug => Self::Debug,
            LogLevel::Trace => Self::Trace,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_app() {
        Opt::command().debug_assert();
    }

    #[test]
    fn file_name_shell() {
        assert_eq!(Shell::Bash.file_name("rzopfli"), "rzopfli.bash");
        assert_eq!(Shell::Elvish.file_name("rzopfli"), "rzopfli.elv");
        assert_eq!(Shell::Fish.file_name("rzopfli"), "rzopfli.fish");
        assert_eq!(Shell::Nushell.file_name("rzopfli"), "rzopfli.nu");
        assert_eq!(Shell::PowerShell.file_name("rzopfli"), "_rzopfli.ps1");
        assert_eq!(Shell::Zsh.file_name("rzopfli"), "_rzopfli");
    }

    #[test]
    fn deref_suffix() {
        assert_eq!(&*Suffix(String::default()), "");
    }

    #[test]
    fn from_str_suffix() {
        assert_eq!(Suffix::from_str(".gz").unwrap(), Suffix(".gz".into()));
    }

    #[test]
    fn from_str_suffix_with_empty_string() {
        assert!(
            Suffix::from_str("")
                .unwrap_err()
                .to_string()
                .contains("the suffix is an empty string")
        );
    }

    #[test]
    fn from_str_suffix_with_path_separator() {
        let suffix = if cfg!(windows) { r"foo\bar" } else { "foo/bar" };
        assert!(
            Suffix::from_str(suffix)
                .unwrap_err()
                .to_string()
                .contains("the suffix contains a path separator")
        );
    }

    #[test]
    fn from_str_suffix_not_starts_with_dot() {
        assert!(
            Suffix::from_str("gz")
                .unwrap_err()
                .to_string()
                .contains("the suffix does not starts with `.`")
        );
    }

    #[test]
    fn default_format() {
        assert_eq!(Format::default(), Format::Gzip);
    }

    #[test]
    fn from_format_to_zopfli_format() {
        assert!(matches!(
            zopfli::Format::from(Format::Gzip),
            zopfli::Format::Gzip
        ));
        assert!(matches!(
            zopfli::Format::from(Format::Zlib),
            zopfli::Format::Zlib
        ));
        assert!(matches!(
            zopfli::Format::from(Format::Deflate),
            zopfli::Format::Deflate
        ));
    }

    #[test]
    fn default_log_level() {
        assert_eq!(LogLevel::default(), LogLevel::Info);
    }

    #[test]
    fn from_log_level_to_level_filter() {
        assert_eq!(LevelFilter::from(LogLevel::Off), LevelFilter::Off);
        assert_eq!(LevelFilter::from(LogLevel::Error), LevelFilter::Error);
        assert_eq!(LevelFilter::from(LogLevel::Warn), LevelFilter::Warn);
        assert_eq!(LevelFilter::from(LogLevel::Info), LevelFilter::Info);
        assert_eq!(LevelFilter::from(LogLevel::Debug), LevelFilter::Debug);
        assert_eq!(LevelFilter::from(LogLevel::Trace), LevelFilter::Trace);
    }
}
