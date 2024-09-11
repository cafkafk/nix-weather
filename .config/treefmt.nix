# SPDX-FileCopyrightText: 2024 Christina Sørensen
# SPDX-FileContributor: Christina Sørensen
#
# SPDX-License-Identifier: EUPL-1.2
{
  projectRootFile = "Cargo.toml";
  programs = {
    nixfmt.enable = true; # nix
    statix.enable = true; # nix static analysis
    deadnix.enable = true; # find dead nix code
    rustfmt.enable = true; # rust
    shellcheck.enable = true; # bash/shell
    taplo.enable = false; # toml
    yamlfmt.enable = true; # yaml
  };
  settings = {
    formatter = {
      shellcheck.excludes = [ ".envrc" ];
    };
  };
}
