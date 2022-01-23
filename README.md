# TABSET
[![Crates.io](https://img.shields.io/crates/v/tabset)](https://crates.io/crates/tabset) 
[![Build](https://github.com/Ewpratten/tabset/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/tabset/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/tabset/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/tabset/actions/workflows/clippy.yml)
[![Audit](https://github.com/Ewpratten/tabset/actions/workflows/audit.yml/badge.svg)](https://github.com/Ewpratten/tabset/actions/workflows/audit.yml)


`tabset` is a Linux command-line tool for configuring your drawing tablets. 

The tool is based around the concept of configuration scripts, and follows a super simple, yet extensible configuration system. Each device you might own gets its own directory, and inside some simple metadata is stored along with a list of shell scripts you can use to set various `xsetwacom` options.

## Installation

[![Get Appimage](https://raw.githubusercontent.com/srevinsaju/get-appimage/master/static/badges/get-appimage-branding-blue.png)](https://github.com/Ewpratten/tabset/releases/latest)

This crate can also be installed via `cargo` with:

```sh
cargo install tabset
```

## Configuration

Configuration files are stored in `~/.config/tabset/`. These files are simply sorted by directory. Here is a listing of my personal configs:

```shell
# ewpratten@ewpratten-desktop ~/.config/tabset $ tree
.
└── devices
    └── huion-kamvas-13
        ├── name
        ├── peripherals
        └── profiles
            ├── blender.sh
            ├── default.sh -> blender.sh
            └── disabled.sh

3 directories, 5 files
```

The `name` and `peripherals` files must exist for each device. `name` is mostly un-used at the moment, but should contain a friendly name for your device. `peripherals` is a list of strings that `tabset` will search for in the output of `xsetwacom --list` if you run the `tabset check <device>` command.

Each of the profile scripts just contain whatever configuration commands you want. You can see my blender one [in the example config directory](./example_configs/devices/huion-kamvas-13/profiles/blender.sh).

### Usage

```text
tabset 0.1.0
Evan Pratten <ewpratten@gmail.com>
A Linux drawing tablet configuration tool

USAGE:
    tabset <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    check      Check if a specific tablet is connected. Prints message and returns status code
    help       Prints this message or the help of the given subcommand(s)
    list       List all configured devices and their profiles
    profile    Set a specific tablet to use a specific profile
```
