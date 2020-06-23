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

/// Get the `GIT_DIR` of the real repository
pub fn real_git_dir() -> Option<PathBuf> {
  let raw = git(&["rev-parse", "--absolute-git-dir"])?;
  Some(Path::new(&raw).to_path_buf())
}

/// Get the `GIT_DIR` of the CommitSync repository
pub fn cs_git_dir() -> Option<PathBuf> {
  let mut path = real_git_dir()?;
  path.push("CommitSync");
  Some(path)
}
