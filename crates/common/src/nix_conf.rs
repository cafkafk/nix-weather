// SPDX-FileCopyrightText: 2024 Christina Sørensen
//
// SPDX-License-Identifier: EUPL-1.2

//! Read `nix.conf` for settings

use std::path::Path;

use thiserror::Error;

pub enum Value<'conf> {
  /// Sets a conf value to this value
  ///
  /// e.g:
  ///
  /// ```conf
  /// keep-outputs = true
  /// ```
  ///
  /// Turns into:
  ///
  /// ```rust
  /// Value::Set { key: "keep-outputs", value: "true" }
  /// ```
  Set { key: &'conf str, value: &'conf str },

  /// Appends to a set value
  ///
  /// e.g:
  ///
  /// ```conf
  /// substituters = a b
  /// extra-substituters = c d
  /// ```
  ///
  /// Turns into:
  ///
  /// ```rust
  /// [
  ///   Value::Set { key: "substituters", value: "a b" },
  ///   Value::Append { key: "substituters", value: "c d" },
  /// ]
  /// ```
  Append { key: &'conf str, value: &'conf str },

  /// Includes another file (error if missing):
  ///
  /// e.g:
  ///
  /// ```conf
  /// include path/to/file
  /// ```
  ///
  /// Turns into:
  ///
  /// ```rust
  /// Value::Include { path: std::path::Path::new("path/to/file") }
  /// ```
  Include { path: &'conf Path },

  /// Includes another file if it exists:
  ///
  /// e.g:
  ///
  /// ```conf
  /// !include path/to/file
  /// ```
  ///
  /// Turns into:
  ///
  /// ```rust
  /// Value::IncludeIfPresent { path: std::path::Path::new("path/to/file") }
  /// ```
  IncludeIfPresent { path: &'conf Path },
}

#[derive(Debug, Error)]
#[error("configuration contained an invalid line: `{invalid_line}`")]
pub struct LineError {
  invalid_line: Box<str>,
}

#[derive(Debug, Error)]
pub enum GetValueError {
  #[error("configuration contained an invalid line: `{line}`")]
  Invalid { line: Box<str> },

  #[error("configuration contained an include to {} which wasn't resolved", path.display())]
  UnresolvedInclude { path: Box<Path> },
}

#[derive(Debug, Error)]
pub enum CliError {
  #[error(transparent)]
  FailedToRunCommand(std::io::Error),

  #[error("command failed with code {status} and stderr:\n{stderr}")]
  CommandFailed {
    status: std::process::ExitStatus,
    stderr: String,
  },

  #[error(transparent)]
  InvalidUtf8(std::string::FromUtf8Error),
}

/// Fetch the configuration from `nix config show`
///
/// Requires `experimental-features = nix-command`
pub fn nix_conf_from_cli() -> Result<String, CliError> {
  // Invoke nix command
  let std::process::Output {
    status,
    stdout,
    stderr,
  } = std::process::Command::new("nix")
    .args(["conf", "show"])
    .output()
    .map_err(CliError::FailedToRunCommand)?;

  // command failed
  if !status.success() {
    return Err(CliError::CommandFailed {
      status,
      stderr: String::from_utf8_lossy(&stderr).into_owned(),
    });
  }

  // convert to string
  String::from_utf8(stdout).map_err(CliError::InvalidUtf8)
}

/// Gets the value of a single key from the provided configuration.
///
/// **Warning:** we normalize the key values by removing the `extra-` prefix
///
/// It will perform value merging as appropriate, but it requires `conf` to have access to *all* set
/// values in order to be correct:
///
/// ```conf
/// # file-1.conf
/// key = a
/// extra-key = b
/// include file-2.conf
/// ```
///
/// ```conf
/// # file-2.conf
/// key = c
/// extra-key = d
/// ```
///
/// The real value is: `key = c b d`
///
/// `get_value(file-1.conf, "key") => Error include file-2.conf not resolved`
/// `get_value(file-2.conf, "key") => c d`
pub fn get_value(conf: &str, key: &str) -> Result<String, GetValueError> {
  let values = conf
    .lines()
    .filter_map(|line| match parse_line(line) {
      Ok(value) => match value? {
        // key matches
        value @ Value::Set {
          key: key_,
          value: _,
        }
        | value @ Value::Append {
          key: key_,
          value: _,
        } if key_ == key => Some(Ok(value)),

        // Key doesn't match
        Value::Set { key: _, value: _ } | Value::Append { key: _, value: _ } => None,

        // we don't resolve includes
        Value::Include { path } | Value::IncludeIfPresent { path } => {
          Some(Err(GetValueError::UnresolvedInclude { path: path.into() }))
        }
      },
      Err(LineError { invalid_line }) => Some(Err(GetValueError::Invalid { line: invalid_line })),
    })
    .collect::<Result<Vec<Value>, GetValueError>>()?;

  let mut final_value = String::new();

  // find the Value::Set that was last in the config (latter value replace the previous one)
  if let Some(value) = values.iter().rev().find_map(|value| match value {
    Value::Set { key: _, value } => Some(value),
    Value::Append { key: _, value: _ } => None,

    // includes being present is an error
    Value::Include { path: _ } | Value::IncludeIfPresent { path: _ } => unreachable!(),
  }) {
    final_value.push_str(value);
  }

  // append Value::Append values to the previous value
  for value in values {
    match value {
      Value::Append { key: _, value } => {
        // add a leading space to create a space separated list
        if !final_value.is_empty() {
          final_value.push(' ');
        }

        final_value.push_str(value);
      }

      // we already handled the Value::Set
      Value::Set { key: _, value: _ } => continue,

      // includes being present is an error
      Value::Include { path: _ } | Value::IncludeIfPresent { path: _ } => unreachable!(),
    }
  }

  Ok(final_value)
}

/// Parse a single line of a nix.conf configuration file
fn parse_line(line: &str) -> Result<Option<Value>, LineError> {
  // strip comments
  let line = line
    .split_once('#')
    .map(|(line, _comment)| line)
    .unwrap_or(line);

  // strip extra whitesapce
  let line = line.trim();

  // return early on empty line
  if line.is_empty() {
    return Ok(None);
  }

  match line.split_once('=') {
    // key = value pair
    Some((key, value)) => {
      // remove extra whitespace
      let key = key.trim();
      let value = value.trim();

      // Are we appending to the previous value?
      match key.strip_prefix("extra-") {
        None => Ok(Some(Value::Set { key, value })),
        Some(key) => Ok(Some(Value::Append { key, value })),
      }
    }

    None => match line.split_once(' ') {
      // include path
      Some(("include", path)) => Ok(Some(Value::Include {
        path: Path::new(path),
      })),
      // !include path
      Some(("!include", path)) => Ok(Some(Value::IncludeIfPresent {
        path: Path::new(path),
      })),

      // Invalid line
      Some(_) | None => Err(LineError {
        invalid_line: line.into(),
      }),
    },
  }
}
