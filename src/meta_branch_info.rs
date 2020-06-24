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
  pub upstream_ref: String,
  pub upstream_sha: String,
  pub upstream_url: String,
  pub hostname: String,
  pub user: String,
  pub meta_committed_at: Option<DateTime<FixedOffset>>,
}

fn cat_file(meta_ref: &str, path: &str) -> Result<String, GitError> {
  cs_git(&["cat-file", "blob", &format!("{}:{}", meta_ref, path)])
}

pub fn meta_branch_info(meta_ref: &str) -> Result<BranchMetadata, GitError> {
  Ok(BranchMetadata {
    commit_sha: cat_file(meta_ref, "commit.sha")?,
    commit_ref: cat_file(meta_ref, "commit.ref")?,
    upstream_ref: cat_file(meta_ref, "upstream.ref")?,
    upstream_sha: cat_file(meta_ref, "upstream.sha")?,
    upstream_url: cat_file(meta_ref, "upstream.url")?,
    hostname: cat_file(meta_ref, "hostname")?,
    user: cat_file(meta_ref, "user")?,
    meta_committed_at: Some(get_cs_commit(meta_ref)?.committed_at),
  })
}

impl BranchMetadata {
  pub fn from_current_commit() -> Result<BranchMetadata, GitError> {
    let upstream_ref = get_upstream()?.expect("Expected a branch");

    Ok(BranchMetadata {
      commit_sha: git(&["rev-parse", "HEAD"])?,
      commit_ref: git(&["symbolic-ref", "HEAD"])?,
      upstream_ref: upstream_ref.to_string(),
      upstream_sha: git(&["rev-parse", &upstream_ref])?,
      upstream_url: git(&[
        "config",
        &format!(
          "remote.{}.url",
          upstream_ref.split("/").collect::<Vec<&str>>()[2]
        ),
      ])?,
      hostname: gethostname::gethostname().into_string().unwrap(),
      user: whoami::username(),
      meta_committed_at: None,
    })
  }
}
