# rinstall
[![Build Status](https://app.travis-ci.com/MatanyaLoewenthal/rinstall.svg?branch=main)](https://app.travis-ci.com/MatanyaLoewenthal/rinstall)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/MatanyaLoewenthal/rinstall)

`rinstall` is a Rust CLI application for reinstalling programs after upgrading or changing a Linux/Unix based Operating System.

Currently, rinstall supports `apt` and `snap` packages installed from the main repositories, but more package managers are planned. see the development progress tab for more info.

## Usage

Usage is simple, just run `rinstall capture <yaml file name> [--manager [apt | snap | all]]` and rinstall will capture all packages installed by the specified package manager(s) in the specified yaml. On the new OS, run the command ` rinstall apply <yaml file name> [--manager [apt | snap | all]]` and rinstall will read the yaml file and install the apropriate applications from the specified manager(s).

Currently, the menu options are as follows:

```bash
rinstall 0.1.0
Matanya Loewenthal
A tool for reinstalling packages

USAGE:
    rinstall <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    apply      Apply this input file to your system
    capture    Capture your system configuration
    help       Print this message or the help of the given subcommand(s)

```
For `rinstall apply`:

```bash
rinstall-apply 0.1.0
Matanya Loewenthal
Apply this input file to your system

USAGE:
    rinstall apply <INPUT>

ARGS:
    <INPUT>    Apply this input file to your system

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

For `rinstall capture`:

```bash
rinstall-capture 0.1.0
Matanya Loewenthal
Capture your system configuration

USAGE:
    rinstall capture [OPTIONS] <OUTPUT>

ARGS:
    <OUTPUT>    Capture your system configuration

OPTIONS:
    -h, --help                    Print help information
    -m, --manager <MANAGER>...    The package manager to capture
    -V, --version                 Print version information
```

## Installation

Thsi project is still in alpha stage, so to install, clone this repo, `cd` inside and run:

```bash
$ cargo run -- [help | apply | capture]
```

## Goals

### Package Managers

| Manager | Progress |
|-------|-------|
apt | Implemented
snap | Implemented
flatpack | Planned
rpm | Planned
dnf | Planned
yum | Planned

### To-Do List

- [ ] Repository Capturing and Applying
- [ ] Testing
- [ ] Documentation
- [ ] Publish
  - [ ] apt
  - [ ] snap
  - [ ] crates.io

