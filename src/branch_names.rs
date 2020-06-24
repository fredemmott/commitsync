/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use crate::{git::*, *};

pub fn create_meta_branch_name() -> Result<String, GitError> {
  let remote_ref =
    git::get_upstream()?.expect("Need an upstream before using CommitSync");
  let remote_branch = remote_ref.split("/").last().expect("Malformed ref");
  let remote_shorthash = git::git(&["rev-parse", "--short", &remote_ref])?;

  let head_shorthash = git::git(&["rev-parse", "--short", "HEAD"])?;
  let head_info =
    git::get_real_commit(&head_shorthash).expect("couldnt read HEAD");
  let head_date = head_info.committed_at.format("%Y-%m-%d");

  Ok(format!(
    "csmeta-{}-{}-{}-{}",
    head_date, remote_branch, remote_shorthash, head_shorthash,
  ))
}

pub fn create_branch_name() -> Result<String, GitError> {
  Ok(format!("cs-{}", create_meta_branch_name()?[7..].to_string()))
}

fn get_meta_for_commit(
  commitish: &str,
) -> Result<(String, BranchMetadata), GitError> {
  let cs_ref = git::cs_git(&[
    "for-each-ref",
    "--points-at",
    commitish,
    "--format=%(refname)",
    "refs/heads/cs-*",
  ])?;
  let meta_ref = format!("refs/heads/csmeta-{}", cs_ref.split("/cs-").last().unwrap());
  let meta = meta_branch_info(&meta_ref)?;
  Ok((meta_ref, meta))
}

pub fn get_meta_branch_name() -> Result<String, GitError> {
  // Re-use a branch if all of:
  // - the original branch name is the same
  // - author username is the same
  // - hostname is the same
  // - it was either attached to the current commit already or the parent
  //
  // Common case is the parent; current commit is generally when debugging, or if not
  // using the commit hook.
  let head_ref = git::git(&["symbolic-ref", "HEAD"])?;
  let user = whoami::username();
  let host = gethostname::gethostname().into_string().unwrap();
  let reuse_key = (head_ref, user, host);

  let head = git::git(&["rev-parse", "HEAD"])?;
  match get_meta_for_commit(&head) {
    Ok((meta_ref, meta)) => {
      if (meta.commit_ref, meta.user, meta.hostname) == reuse_key {
        return Ok(meta_ref.split("/").last().unwrap().to_string());
      }
    }
    Err(_) => (),
  }

  let parent = git::git(&["rev-parse", "HEAD^"])?;
  match get_meta_for_commit(&parent) {
    Ok((meta_ref, meta)) => {
      if (meta.commit_ref, meta.user, meta.hostname) == reuse_key {
        return Ok(meta_ref.split("/").last().unwrap().to_string());
      }
    }
    Err(_) => (),
  }

  create_meta_branch_name()
}

pub fn get_branch_name() -> Result<String, GitError> {
  Ok(format!("cs-{}", get_meta_branch_name()?[7..].to_string()))
}
