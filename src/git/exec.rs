/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use subprocess::{Exec, Redirection};

// Specific to avoid a circular dependency
use crate::git::dirs::cs_git_dir;

use crate::Result;

/// Execute `git` in the real repository.
pub fn git(args: &[&str]) -> Result<String> {
  let out = Exec::cmd("git")
    .args(args)
    .stdout(Redirection::Pipe)
    .stderr(Redirection::Merge)
    .capture()?
    .stdout_str()
    .trim()
    .to_string();
  Ok(out)
}

/// Execute `git` in the CommitSync repository.
pub fn cs_git(args: &[&str]) -> Result<String> {
  let git_dir = cs_git_dir()?;
  let mut index_file = git_dir.clone();
  index_file.push("index");

  let out = Exec::cmd("git")
    .args(args)
    .stdout(Redirection::Pipe)
    .stderr(Redirection::Merge)
    .env("GIT_DIR", &git_dir)
    .env("GIT_INDEX_FILE", &index_file)
    .capture()?
    .stdout_str()
    .trim()
    .to_string();
  Ok(out)
}
