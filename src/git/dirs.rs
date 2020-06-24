/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use std::path::{Path, PathBuf};

// Specific to avoid a circular dependency
use crate::git::exec::git;

static mut CACHED_GIT_DIR: Option<PathBuf> = None;

/// Get the `GIT_DIR` of the real repository
pub fn real_git_dir() -> Result<PathBuf, crate::GitError> {
  match unsafe { &CACHED_GIT_DIR } {
    Some(dir) => Ok(dir.to_path_buf()),
    None => {
      let path = git(&["rev-parse", "--absolute-git-dir"])?;
      let buf = Path::new(&path).to_path_buf();
      unsafe { CACHED_GIT_DIR = Some(buf.to_path_buf()) };
      Ok(buf)
    }
  }
}

/// Get the `GIT_DIR` of the CommitSync repository
pub fn cs_git_dir() -> Result<PathBuf, crate::GitError> {
  let mut path = real_git_dir()?;
  path.push("CommitSync");
  Ok(path)
}
