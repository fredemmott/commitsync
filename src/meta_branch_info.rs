/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use crate::git::*;
use chrono::prelude::*;

#[derive(Debug)]
pub struct BranchMetadata {
  pub commit_ref: String,
  pub commit_sha: String,
  pub upstream_sha: String,
  pub hostname: String,
  pub user: String,
  pub meta_committed_at: DateTime<FixedOffset>,
}

fn cat_file(meta_ref: &str, path: &str) -> Result<String, GitError> {
  cs_git(&["cat-file", "blob", &format!("{}:{}", meta_ref, path)])
}

pub fn meta_branch_info(meta_ref: &str) -> Result<BranchMetadata, GitError> {
  Ok(BranchMetadata {
    commit_sha: cat_file(meta_ref, "commit.sha")?,
    commit_ref: cat_file(meta_ref, "commit.ref")?,
    upstream_sha: cat_file(meta_ref, "upstream.sha")?,
    hostname: cat_file(meta_ref, "hostname")?,
    user: cat_file(meta_ref, "user")?,
    meta_committed_at: get_cs_commit(meta_ref)?.committed_at,
  })
}
