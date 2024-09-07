# SPDX-FileCopyrightText: 2024 Christina Sørensen
#
# SPDX-License-Identifier: EUPL-1.2

{
  description = "Nix Weather - Check Cache Availablility of NixOS Configurations";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    lix-module = {
      url = "git+https://git.lix.systems/lix-project/nixos-module?ref=main";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    gitignore = {
      url = "github:hercules-ci/gitignore.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs-stable.url = "github:NixOS/nixpkgs/nixos-24.05";

    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };

    pre-commit-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        nixpkgs-stable.follows = "nixpkgs-stable";
        flake-compat.follows = "flake-compat";
        gitignore.follows = "gitignore";
      };
    };

  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      treefmt-nix,
      fenix,
      flake-utils,
      rust-overlay,
      advisory-db,
      pre-commit-hooks,
      lix-module,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];

        pkgs = (import nixpkgs) { inherit system overlays; };

        inherit (pkgs) lib;

        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./.config/rust-toolchain.toml;

        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
        src = craneLib.cleanCargoSource ./.;

        # Common arguments can be set here to avoid repeating them later
        commonArgs = {
          inherit src;
          strictDeps = true;

          nativeBuildInputs = [ pkgs.pkg-config ];

          buildInputs =
            [
              pkgs.openssl
              pkgs.installShellFiles
            ]
            ++ lib.optionals pkgs.stdenv.isDarwin [
              # Additional darwin specific inputs can be set here
              pkgs.libiconv
            ];

          # Additional environment variables can be set directly
          # MY_CUSTOM_VAR = "some value";
        };

        craneLibLLvmTools = craneLib.overrideToolchain (
          fenix.packages.${system}.complete.withComponents [
            "cargo"
            "llvm-tools"
            "rustc"
          ]
        );

        # Build *just* the cargo dependencies (of the entire workspace),
        # so we can reuse all of that work (e.g. via cachix) when running in CI
        # It is *highly* recommended to use something like cargo-hakari to avoid
        # cache misses when building individual top-level-crates
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        individualCrateArgs = commonArgs // {
          inherit cargoArtifacts;
          inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;
          # NB: we disable tests since we'll run them all via cargo-nextest
          doCheck = false;
        };

        fileSetForCrate =
          crate:
          lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              ./crates/common
              ./crates/workspace-hack
              crate
            ];
          };

        # Build the top-level crates of the workspace as individual derivations.
        # This allows consumers to only depend on (and build) only what they need.
        # Though it is possible to build the entire workspace as a single derivation,
        # so this is left up to you on how to organize things
        nix-weather = craneLib.buildPackage (
          individualCrateArgs
          // {
            pname = "nix-weather";
            cargoExtraArgs = "-p nix-weather";
            src = fileSetForCrate ./crates/nix-weather;
            MAN_OUT = "./man";

            preInstall = ''
              cd crates/nix-weather
              installManPage man/nix-weather.1
              installShellCompletion \
                --fish man/nix-weather.fish \
                --bash man/nix-weather.bash \
                --zsh  man/_nix-weather
              mkdir -p $out
              cd ../..
            '';

            meta = {
              mainProgram = "nix-weather";
            };
          }
        );

        treefmtEval = treefmt-nix.lib.evalModule pkgs .config/treefmt.nix;
      in
      {
        formatter = treefmtEval.config.build.wrapper;

        checks = {
          # Build the crates as part of `nix flake check` for convenience
          inherit nix-weather;

          # Run clippy (and deny all warnings) on the workspace source,
          # again, reusing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          cargo-workspace-clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            }
          );

          cargo-workspace-doc = craneLib.cargoDoc (commonArgs // { inherit cargoArtifacts; });

          # Check formatting
          cargo-workspace-fmt = craneLib.cargoFmt { inherit src; };

          # Audit dependencies
          cargo-workspace-audit = craneLib.cargoAudit { inherit src advisory-db; };

          # Audit licenses
          cargo-workspace-deny = craneLib.cargoDeny { inherit src; };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on other crate derivations
          # if you do not want the tests to run twice
          cargo-workspace-nextest = craneLib.cargoNextest (
            commonArgs
            // {
              inherit cargoArtifacts;
              partitions = 1;
              partitionType = "count";
            }
          );

          # Ensure that cargo-hakari is up to date
          cargo-workspace-hakari = craneLib.mkCargoDerivation {
            inherit src;
            pname = "cargo-workspace-hakari";
            cargoArtifacts = null;
            doInstallCargoArtifacts = false;

            buildPhaseCargoCommand = ''
              cargo hakari generate --diff  # workspace-hack Cargo.toml is up-to-date
              cargo hakari manage-deps --dry-run  # all workspace crates depend on workspace-hack
              cargo hakari verify
            '';

            nativeBuildInputs = [ pkgs.cargo-hakari ];
          };
          pre-commit-check =
            let
              # some treefmt formatters are not supported in pre-commit-hooks we filter them out for now.
              toFilter = [ "yamlfmt" ];
              filterFn = n: _v: (!builtins.elem n toFilter);
              treefmtFormatters = pkgs.lib.mapAttrs (_n: v: { inherit (v) enable; }) (
                pkgs.lib.filterAttrs filterFn (import ./.config/treefmt.nix).programs
              );
            in
            pre-commit-hooks.lib.${system}.run {
              src = ./.;
              hooks = treefmtFormatters // {
                # not in treefmt
                convco.enable = true;
                reuse = {
                  enable = true;
                  name = "reuse";
                  entry = with pkgs; "${pkgs.reuse}/bin/reuse lint";
                  pass_filenames = false;
                };
              };
            };
          formatting = treefmtEval.config.build.check self;
        };

        packages =
          {
            inherit nix-weather;
            default = nix-weather;
          }
          // lib.optionalAttrs (!pkgs.stdenv.isDarwin) {
            cargo-workspace-llvm-coverage = craneLibLLvmTools.cargoLlvmCov (
              commonArgs // { inherit cargoArtifacts; }
            );
          };

        apps = {
          nix-weather = flake-utils.lib.mkApp { drv = nix-weather; };
        };

        # For `nix develop`:
        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};

          # Additional dev-shell environment variables can be set directly
          # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

          # Extra inputs can be added here; cargo and rustc are provided by default.
          packages = [
            toolchain
            pkgs.fd
            pkgs.rustup
            pkgs.cargo-hakari
            pkgs.reuse
            pkgs.just
            lix-module.packages.${system}.default
          ];

          inherit (self.checks.${system}.pre-commit-check) shellHook;
        };
      }
    );
  nixConfig = {
    experimental-features = [
      "nix-command"
      "flakes"
      "pipe-operator"
      "repl-flake"
    ];
  };
}
