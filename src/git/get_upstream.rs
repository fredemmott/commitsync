/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use crate::git::*;

/** Find the upstream for the current branch.
 *
 * This will walk backwards through the commit history until it finds a commit
 * that is the head of a remote branch.
 */
pub fn get_upstream() -> Result<Option<String>, GitError> {
  let raw_commits = git(&["rev-list", "HEAD"])?;
  if raw_commits.is_empty() {
    return Ok(None);
  }

  let commits = raw_commits.split("\n");
  for commit in commits {
    match git(&[
      "name-rev",
      "--no-undefined",
      "--name-only",
      "--exclude=HEAD",
      "--ref=refs/remotes/*",
      &commit,
    ]) {
      Ok(ref_name) => {
        return Ok(Some(format!(
          "refs/{}",
          ref_name.split("~").nth(0).expect("Couldn't split ref name")
        )))
      }
      _ => (),
    }
  }
  Ok(None)
}
