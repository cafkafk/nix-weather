<!--
SPDX-FileCopyrightText: 2023-2024 Christina Sørensen
SPDX-FileContributor: Christina Sørensen

SPDX-License-Identifier: EUPL-1.2
-->

<div style="padding-left: 15%;padding-right: 15%;">
<div align="center">

# Weather - Check Cache Availablility of NixOS Configurations

A *fast* rust tool to check availability of your entire system in caches. It so
to speak "checks the weather" before going to update. Heavily inspired by [`guix
weather`](https://guix.gnu.org/manual/en/html_node/Invoking-guix-weather.html).


[![Built with Nix](https://img.shields.io/badge/Built_With-Nix-5277C3.svg?logo=nixos&labelColor=73C3D5)](https://nixos.org)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)
[![REUSE status](https://api.reuse.software/badge/git.fsfe.org/reuse/api)](https://api.reuse.software/info/git.fsfe.org/reuse/api)
[![License: EUPL-1.2](https://img.shields.io/badge/licence-EUPL--1.2-blue)](https://commission.europa.eu/content/european-union-public-licence_en)

</div>


## Usage

> **Note** 
> Currently, `nix-weather` only has first-class support for flakes.

General usage would be like this:

```bash
nix-weather --name myhost --config ~/git/my-nixos-config
```

Here, we specify the name of the host, as specified at the flake output
`nixosConfiguration`, and a path to the NixOS configuration flake.

## How It Works

The basic idea is that we construct a set of all requisites to build the
top-level of a NixOS configuration, and then query Nix cache(s) for the narinfo.
By doing this in a high concurrency, parallel task runner (i.e. tokio async
runtime), and only querying the headers for status codes, we can reach
impressive speeds, typically around 45~ network time. 

One of the biggest limiting factors regarding speed is building the
`config.system.toplevel`, and finding the necessary requisites with `nix-store`.
Caching the requisites is a future goal, so that we only have to build the
`toplevel`, and then match against its derivation in cache, which should cut
down the nix part of the runtime by ~80%.

</div>
