/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use crate::git::git;

/** Find the upstream for the current branch.
 *
 * This will walk backwards through the commit history until it finds a commit
 * that is the head of a remote branch.
 */
pub fn get_upstream() -> Option<String> {
  let raw_refs = git(&[
    "for-each-ref",
    "--format=%(objectname) %(refname)",
    "refs/remotes/",
  ])?;
  if raw_refs.is_empty() {
    return None;
  }

  let raw_commits = git(&["rev-list", "HEAD"])?;
  if raw_commits.is_empty() {
    return None;
  }

  let refs: Vec<(&str, &str)> = raw_refs
    .split("\n")
    .map(|line| {
      let parts: Vec<&str> = line.split(" ").collect();
      (parts[0], parts[1])
    })
    .collect();

  let commits = raw_commits.split("\n");
  for commit in commits {
    match refs.iter().find(|(ref_commit, _)| ref_commit == &commit) {
      Some((_, refname)) => return Some(refname.to_string()),
      None => (),
    }
  }
  None
}
