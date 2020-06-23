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

/// Execute `git` in the real repository.
pub fn git(args: &[&str]) -> Option<String> {
  match Exec::cmd("git")
    .args(args)
    .stdout(Redirection::Pipe)
    .stderr(Redirection::Merge)
    .capture()
  {
    Ok(data) => {
      let str = data.stdout_str();
      Some(str.trim().to_string())
    }
    Err(_) => None,
  }
}

/// Execute `git` in the CommitSync repository.
pub fn cs_git(args: &[&str]) -> Option<String> {
  let git_dir = cs_git_dir()?;
  let mut index_file = git_dir.clone();
  index_file.push("index");

  match Exec::cmd("git")
    .args(args)
    .stdout(Redirection::Pipe)
    .stderr(Redirection::Merge)
    .env("GIT_DIR", &git_dir)
    .env("GIT_INDEX_FILE", &index_file)
    .capture()
  {
    Ok(data) => {
      let str = data.stdout_str();
      Some(str.trim().to_string())
    }
    Err(_) => None,
  }
}
