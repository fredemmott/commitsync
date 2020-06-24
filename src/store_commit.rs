/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use crate::{branch_names::*, git::*, *};
use std::io::prelude::*;
use tempfile::NamedTempFile;

fn stage_file(name: &str, content: &str) -> Result<(), GitError> {
  let mut tempfile = NamedTempFile::new().unwrap();
  tempfile.write_all(content.as_bytes()).unwrap();
  tempfile.flush().expect("Failed to write blob to temp file");
  let hash = cs_git(&[
    "hash-object",
    "-w",
    tempfile.path().to_str().expect("no path for temp file"),
  ])?;
  cs_git(&[
    "update-index",
    "--add",
    "--cacheinfo",
    &format!("100644,{},{}", hash, name),
  ])?;
  Ok(())
}

fn commit_metadata(cs_meta_ref: &str) -> Result<String, GitError> {
  let meta = BranchMetadata::from_current_commit()?;

  stage_file("commit.sha", &meta.commit_sha)?;
  stage_file("commit.ref", &meta.commit_ref)?;
  stage_file("upstream.ref", &meta.upstream_ref)?;
  stage_file("upstream.sha", &meta.upstream_sha)?;
  stage_file("upstream.url", &meta.upstream_url)?;
  stage_file("hostname", &meta.hostname)?;
  stage_file("user", &meta.user)?;

  let tree = cs_git(&["write-tree"])?;
  let message = format!("Metadata for {}", &meta.commit_sha);

  // Continue a branch or start a new one as an orphan?
  match cs_git(&["show-ref", "--hash", cs_meta_ref]) {
    Ok(meta_parent) => Ok(cs_git(&[
      "commit-tree",
      "-m",
      &message,
      "-p",
      &meta_parent,
      &tree,
    ])?),
    Err(_) => Ok(cs_git(&["commit-tree", "-m", &message, &tree])?),
  }
}

pub fn store_commit() -> Result<(String, String), GitError> {
  let _ = cs_git(&["fetch"]);
  let cs_ref = format!("refs/heads/{}", &get_branch_name()?);
  let cs_meta_ref = format!("refs/heads/{}", &get_meta_branch_name()?);

  let cs_sha = git(&["rev-parse", "HEAD"])?;
  let cs_meta_sha = commit_metadata(&cs_meta_ref)?;

  cs_git(&["update-ref", &cs_meta_ref, &cs_meta_sha])?;
  cs_git(&["update-ref", &cs_ref, &cs_sha])?;

  cs_git(&["push", "commitsync", &cs_ref, &cs_meta_ref])?;

  Ok((cs_ref, cs_meta_ref))
}
