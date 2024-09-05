<!--
SPDX-FileCopyrightText: 2023-2024 Christina Sørensen
SPDX-FileContributor: Christina Sørensen

SPDX-License-Identifier: EUPL-1.2
-->

# Contributing to Nix Weather

Nix Weather is a NixOs adjacent project, and expects contributors to have **at least** a local installation of `Nix` or `Lix` on their development machine.

## DevShell

Currently, `nix-weather` is developed against Lix main branch. We also make use
of experimental features, including ones still in RFC stage, such as [RFC 0148:
`pipe-operators`](https://github.com/NixOS/rfcs/pull/148).

The `devShell` in the flake takes care of setting this up for you, so as long as
you can run that, you shouldn't have to worry.

Further, the devShell includes formatting through `nix fmt`, and testing through
`nix flake check`, as well as pre-commit-hooks, that ensure any PR is up to
standard before being submitted. 

We stronly advice working and commiting from inside the devShell. To make this
easier, use the provided [direnv](https://direnv.net/) configuration by
installing direnv and running `direnv allow` in the repository root.

## Code Standards

We make use of several standards, including:
- [Semantic Versioning](https://semver.org/) for version bumps.
- [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) for commit summaries.
- [REUSE](https://reuse.software/) for license complicance.

We also make use of code formatters, as well as rust related tooling for
auditing dependencies have licenses that are compliant with the
[EUPL-1.2](https://commission.europa.eu/content/european-union-public-licence_en).

Take note that contributions will be legally considered under the EUPL-1.2
Article 6 "Chain of Authorship" section, essentially a developer certificate of
origin.

> 6.Chain of Authorship
>
> The original Licensor warrants that the copyright in the Original Work granted
> hereunder is owned by him/her or licensed to him/her and that he/she has the
> power and authority to grant the Licence.
>
> Each Contributor warrants that the copyright in the modifications he/she
> brings to the Work are owned by him/her or licensed to him/her and that he/she
> has the power and authority to grant the Licence.
>
> Each time You accept the Licence, the original Licensor and subsequent
> Contributors grant You a licence to their contributions to the Work, under the
> terms of this Licence.

### Pull-Requests

We currently expect that all commits are functional against `nix flake check`
and don't introduce any regressions. Further we prohibit merge branch updates,
and expect all pull requests to have been rebased against the `main` branch
before being merged.

Long chains of commits should be divided into separate PRs, specially if
introducing multiple features that  will all need separate review. Ideally, a
stacked workflow is preferred.

Formatting changes of code created in the PR should be rebased into the commit
introducing the codebase, **not** be it's own separate commit.

Treewide changes should be added to a `.git-blame-ignore-revs`.

## Code of Conduct

The project is moderated according to the [Contributor Covenant Code of
Conduct](CODE_OF_CONDUCT.md). We expect all contributions, issues, and other
project communications made in spaces related to the project to live up to the
standards set in the code of conduct, and it will be enforced according to the
`Enforcement Guidelines` it describes.

Instances of abusive, harassing, or otherwise unacceptable behavior may be
reported to the community leaders responsible for enforcement at:

matrix: @cafkafk:gitter.im
