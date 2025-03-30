<!--
SPDX-FileCopyrightText: 2024 Shun Sakai

SPDX-License-Identifier: Apache-2.0 OR MIT
-->

# rzopfli

[![CI][ci-badge]][ci-url]
[![Version][version-badge]][version-url]
![MSRV][msrv-badge]
![License][license-badge]

**rzopfli** is a lossless data compression tool which uses the [Zopfli]
compression algorithm.

![Demo animation](assets/demo.gif)

## Installation

### From source

```sh
cargo install rzopfli
```

### From binaries

The [release page] contains pre-built binaries for Linux, macOS and Windows.

### How to build

Please see [BUILD.adoc].

## Usage

The command line syntax of `rzopfli` is similar to `gzip` and `zstd`, and some
options derived from the Zopfli reference implementation. `rzopfli` preserves
input files by default, just like `zstd`. It's possible to remove them
automatically by using `--rm`.

### Basic usage

Compress a file into the gzip format:

```sh
rzopfli foo.txt
```

Write the processed data to standard output:

```sh
rzopfli -c foo.txt
```

Remove an input file after successful compression:

```sh
rzopfli --rm foo.txt
```

Performs 50 compression iterations:

```sh
rzopfli -i 50 foo.txt
```

Compress a file into the zlib format:

```sh
rzopfli --format zlib foo.txt
```

### Generate shell completion

`--generate-completion` option generates shell completions to standard output.

The following shells are supported:

- `bash`
- `elvish`
- `fish`
- `nushell`
- `powershell`
- `zsh`

Example:

```sh
rzopfli --generate-completion bash > rzopfli.bash
```

## Command-line options

Please see the following:

- [`rzopfli(1)`]

## Source code

The upstream repository is available at
<https://github.com/sorairolake/rzopfli.git>.

The source code is also available at:

- <https://gitlab.com/sorairolake/rzopfli.git>
- <https://codeberg.org/sorairolake/rzopfli.git>

## Changelog

Please see [CHANGELOG.adoc].

## Contributing

Please see [CONTRIBUTING.adoc].

## Home page

<https://sorairolake.github.io/rzopfli/>

## License

Copyright (C) 2024 Shun Sakai (see [AUTHORS.adoc])

1.  This program is distributed under the terms of either the _Apache License
    2.0_ or the _MIT License_.
2.  Some files are distributed under the terms of the _Creative Commons
    Attribution 4.0 International Public License_.

This project is compliant with version 3.3 of the [_REUSE Specification_]. See
copyright notices of individual files for more details on copyright and
licensing information.

[ci-badge]: https://img.shields.io/github/actions/workflow/status/sorairolake/rzopfli/CI.yaml?branch=develop&style=for-the-badge&logo=github&label=CI
[ci-url]: https://github.com/sorairolake/rzopfli/actions?query=branch%3Adevelop+workflow%3ACI++
[version-badge]: https://img.shields.io/crates/v/rzopfli?style=for-the-badge&logo=rust
[version-url]: https://crates.io/crates/rzopfli
[msrv-badge]: https://img.shields.io/crates/msrv/rzopfli?style=for-the-badge&logo=rust
[license-badge]: https://img.shields.io/crates/l/rzopfli?style=for-the-badge
[Zopfli]: https://github.com/google/zopfli
[release page]: https://github.com/sorairolake/rzopfli/releases
[BUILD.adoc]: BUILD.adoc
[`rzopfli(1)`]: https://sorairolake.github.io/rzopfli/book/man/man1/rzopfli.1.html
[CHANGELOG.adoc]: CHANGELOG.adoc
[CONTRIBUTING.adoc]: CONTRIBUTING.adoc
[AUTHORS.adoc]: AUTHORS.adoc
[_REUSE Specification_]: https://reuse.software/spec-3.3/
