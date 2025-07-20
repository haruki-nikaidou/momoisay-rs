# momoisay-rs

Momoisay is a CLI program like cowsay, but instead of a talking cow, it's Saiba Momoi from Blue Archive!

Rewrite from [Mon4sm/momoisay](https://github.com/Mon4sm/momoisay).

## Preview

Preview of freestyle mode.

![Preview](./preview.gif)

## Features

- Written in Rust!
- Talking ASCII art of Momoi
- Animated ASCII art of Momoi
- Freestyle changing animation of Momoi

## Installation

### Cargo

```sh
cargo install momoisay
```

### Nix

Quick usage (one-off)

```sh
nix run github:haruki-nikaidou/momoisay-rs -- say "Yuuka is 100kg"
```

Install with flake

```nix
{
  inputs = {
    momoi-say.url  = "github:haruki-nikaidou/momoisay-rs";
  };
}
```


```nix
{ pkgs, ... }: {
  home.packages = [ inputs.momoi-say.packages.${pkgs.system}.momoiSay ];
}
```

### Manually Build

```sh
git clone https://github.com/haruki-nikaidou/momoisay-rs.git
cd momoisay-rs
cargo build -r
```

## Usage

```
Usage: momoisay <COMMAND>

Commands:
  say        Display Momoi saying the provided text
  animate    Display an animated Momoi (variant 1 or 2)
  freestyle  Display Momoi in freestyle mode. Pretty cool for ricing btw
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```