<!--
SPDX-FileCopyrightText: 2023 Christina Sørensen
SPDX-FileContributor: Christina Sørensen

SPDX-License-Identifier: AGPL-3.0-only
-->

# Changelog

## [0.1.13] - 2024-02-01

### Features

- Add one fedi fortune

### Build

- Bump actions/stale from 8 to 9
- Bump DeterminateSystems/nix-installer-action from 8 to 9
- Bump grep from 0.2.12 to 0.3.1
- Bump clap from 4.4.11 to 4.4.18
- Bump clap_complete from 4.4.4 to 4.4.9
- Bump assert_cmd from 2.0.12 to 2.0.13
- Bump tempfile from 3.8.1 to 3.9.0
- Bump clap_mangen from 0.2.15 to 0.2.17
- Change flake inputs

### Ci

- Remove stalebot

## [0.1.12] - 2023-12-07

### Features

- Port 187 lines of computers

### Miscellaneous Tasks

- Release fortune-kind v0.1.12

### Build

- Bump grep-regex from 0.1.11 to 0.1.12
- Bump grep-searcher from 0.1.11 to 0.1.13
- Bump clap from 4.4.8 to 4.4.11

## [0.1.11] - 2023-11-30

### Features

- Port 200 lines of miscellaneous

### Miscellaneous Tasks

- Release fortune-kind v0.1.11

### Build

- Bump DeterminateSystems/nix-installer-action from 7 to 8
- Bump webiny/action-conventional-commits from 1.1.0 to 1.2.0
- Bump grep-matcher from 0.1.6 to 0.1.7

## [0.1.10] - 2023-11-23

### Features

- Port 484 lines of miscellaneous

### Miscellaneous Tasks

- Update flake inputs
- Release fortune-kind v0.1.10

## [0.1.9] - 2023-11-16

### Features

- Port 227 lines of people

### Miscellaneous Tasks

- Release fortune-kind v0.1.9

### Build

- Bump clap from 4.4.7 to 4.4.8

## [0.1.8] - 2023-11-09

### Features

- Fair randomness calculation
- Flake refactor, REUSE compliance
- Port 198 lines of riddles

### Miscellaneous Tasks

- Release fortune-kind v0.1.8

### Build

- Bump DeterminateSystems/nix-installer-action from 6 to 7

## [0.1.7] - 2023-11-02

### Bug Fixes

- Remove fortune I'm unsure about

### Features

- Port 241 lines of medicine

### Miscellaneous Tasks

- Release fortune-kind v0.1.7

### Build

- Bump tempfile from 3.8.0 to 3.8.1
- Change to rime.cx and semnix

## [0.1.6] - 2023-10-26

### Features

- Start porting food
- Add some fedi tune
- Begin porting humorists

### Miscellaneous Tasks

- Update all dependencies
- Release fortune-kind v0.1.6

## [0.1.5] - 2023-10-19

### Features

- Add a fortune
- Add more fortunes
- Add nethack sounds
- Start porting debian

### Miscellaneous Tasks

- Release fortune-kind v0.1.5

### Build

- Bump actions/checkout from 3 to 4
- Bump DeterminateSystems/nix-installer-action from 5 to 6

## [0.1.4] - 2023-10-12

### Bug Fixes

- Removed inaccurate fortune
- Remove unnescesarry joke
- Removed confucious inside fortune cookie due to request
- Remove homophobic fortune
- Add progress marker

### Features

- Transfer a few goedel tunes
- Port more tunes
- Finish porting goedel
- Start porting disclaimer
- Port more disclaimers
- Finish porting disclaimer
- Start porting
- Removed fortuens I weren't sure about
- Port more news
- Port all of magic
- Begin porting linux
- OH and added
- Add more fortunes
- Port more fortunes
- Begin porting kids
- Add fortunes from fedi

### Miscellaneous Tasks

- Release fortune-kind v0.1.4

### Refactor

- Move random to own file
- Move out to own file
- Move to own file

### Build

- Update flake.lock

### Ci

- Create flakehub-publish-tagged.yml

## [0.1.3] - 2023-10-09

### Bug Fixes

- Fix gitignore
- Fix gitignore further
- Fix build, phew :p

### Documentation

- Update flake description

### Miscellaneous Tasks

- Release fortune-kind v0.1.3

### Testing

- Fix tests in ci

## [0.1.2] - 2023-10-09

### Documentation

- Add Nix/NixOS installation instructions
- Fix gh alerts
- Change layout slightly
- Fix Nix/NixOS installation instructions
- Change layout
- Make motivation more clear

### Features

- Install from flake
- Update justfile
- Autogen bash completions
- Add all completion types
- Automatic manpage generation
- Gen/install manpages, completion
- Auto-install shell completions, man pages

### Miscellaneous Tasks

- Release fortune-kind v0.1.2

### Refactor

- Command line input

### Build

- Bump clap from 4.4.4 to 4.4.6
- Bump DeterminateSystems/nix-installer-action from 4 to 5
- Use eza style automagic release

## [0.1.1] - 2023-10-06

### Bug Fixes

- Make io_err panic on default arm

### Documentation

- Create editorial guidelines
- Fix typos
- Fix typo
- Linewrap EDITORIAL.md
- Add revision policy
- Remove empty lines in ordered list
- Wrap lines, add editorial

### Features

- Add pets: deeleted some repeats, removed a story that features animal violence
- Remove a repeat fortune from pets, remove poorly formatted goldfish fortune, remove reference to suicide in pets
- Delete oldtunes/pets
- Introduce FORTUNE_DIR env var

### Miscellaneous Tasks

- Bump version to v0.1.1

### Refactor

- Remove unused module
- Fix some clippy lints
- Fix clippy lint
- Clippy passes
- S/fortunes_dir/fortune_dir/g
- Change error printed for NotFound to be `err` not `io_err`
- Introduce env getters
- Make fortune file error handling reusable
- Move `handle_errors` out of get_quote

### Build

- Bump actions/checkout from 2 to 4
- Bump clap from 4.3.23 to 4.4.4
- Format flake
- Format flake

### Ci

- Add some CI

## [0.1.0] - 2023-09-18

### Documentation

- Add readme to root
- Add bare minimum
- Fix spelling mistake
- Fix spelling mistake
- Update README.md
- $$$$$$$ :D
- Document search module
- Document file module
- Document random module
- Document fortunes module
- Add todos for removal of example code
- Add demo gif
- Update README.md

### Features

- Init search_string
- Add read_all_files
- Add find MVP
- Fortune-mod style search

### Miscellaneous Tasks

- Initial commit
- Add ascii-art fortunes
- Use pratchett fortunes
- Use translate-me fortunes
- Use majority of paradoxum fortunes
- Remove off/fortunes, unfunny
- Remove off/rotated
- Used half of off/art
- Removed off/racism, for obvious reasons
- Use most of off/cookie
- Remove misandry (see reasoning)
- Remove misogyny
- Remove rest of paradoxum
- Use all of tao
- Release 0.1.0

### Refactor

- Add file module
- Introduce modules random, fortune
- Module fortunes -> fortune

### Testing

- Add no_match
- Test file module
- Testing for get_quote
- Add tests for random::random

### Build

- Change name to fortune-kind
- Remove old cmake
- Lock cargo file
- Remove offensive fortunes cmake file
- Add grep (ripgrep)
- Add grep-matcher, grep-regex
- Add tempdir
- Move tempfile to dev-dependencies
- Add dev-dependency assert-cmd
- Add release script

