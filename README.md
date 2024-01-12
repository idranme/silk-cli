# Silk Cli

[![crates.io](https://img.shields.io/crates/v/silk-cli.svg)](https://crates.io/crates/silk-cli)

Tencent SILK codec

## Supported platforms

On macOS, arm64 and x64 are supported. On Linux and Windows, only x64 is supported.

## Installation

Using [Scoop](https://scoop.sh/):
```bash
scoop install https://mirror.ghproxy.com/https://raw.githubusercontent.com/idanran/silk-cli/main/silk-cli.json
```

If you're not using one of the above package managers, go to [Releases](https://github.com/idanran/silk-cli/releases) and download it!

<!--
Using [Nix](https://nixos.org/download.html):
```bash
nix-shell -p silk-cli
```
-->

## Usage

```
Usage: silk-cli [OPTIONS] --input <INPUT> --output <OUTPUT> --sample-rate <SAMPLE_RATE>

Options:
  -i, --input <INPUT>              Path of the input file
  -o, --output <OUTPUT>            Path of the output file
  -s, --sample-rate <SAMPLE_RATE>  Sample rate of the input file
  -m, --mode <MODE>                Operating mode [default: encode] [possible values: encode, decode]
  -h, --help                       Print help
  -V, --version                    Print version
```