# What The Alias

![Screenshot of a terminal window with What The Alias run immediately, printing out: "Did you know? You can use the alias: 'gcam' instead of 'git commit -am'"](preview.png)

Recursively scans shell config files and collects alias definitions in order to:

- remind you of a random alias
- train new aliases with spaced repetition (coming soon)
- search for an alias (coming sooner)

## Shell alias trainer

I put a call to this tool into my `.zshrc`. Every time I open a terminal, I get reminded of one of the aliases I defined. This allows me to make even more use of aliases. In the past I often forgot that I even defined an alias, or what it was exactly.

## Installation

WTA is written in Rust, and you can compile the executable from source via:

`cargo build --release`

You then find the executable at: `target/release/wta`. Include this directory in your system path, or copy/move the executable as you see fit.

## Usage:

Get reminded of a random alias:

`wta reminder --random`

## Supported shells

- Bash (`.bashrc`)
- Zsh (`.zshrc`)

## Roadmap

Upcoming features:

### Search aliases

Rough plan to implement this:

- phase 1: search by calling `wta` with arguments
- phase 2: interactive instant search

### Spaced repetition reminder

Spaced repetition will help enforce the learning of new aliases.

Rough plan to implement this:

- collect and sync found aliases to a local SQLite database
- record last timestamp of last reminder, and count of how often reminded already
