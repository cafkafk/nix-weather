# SPDX-FileCopyrightText: 2024 Christina Sørensen
#
# SPDX-License-Identifier: EUPL-1.2

update-deps:
  cargo hakari generate

changelog:
  git cliff -c .config/cliff.toml
