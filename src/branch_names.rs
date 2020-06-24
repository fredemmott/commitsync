/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use crate::{git::*, *};

pub fn create_branch_name() -> Result<String, GitError> {
  let remote_ref =
    git::get_upstream()?.expect("Need an upstream before using CommitSync");
  let remote_branch = remote_ref.split("/").last().expect("Malformed ref");
  let remote_shorthash = git::git(&["rev-parse", "--short", &remote_ref])?;

  let head_shorthash = git::git(&["rev-parse", "--short", "HEAD"])?;
  let head_info =
    git::get_real_commit(&head_shorthash).expect("couldnt read HEAD");
  let head_date = head_info.committed_at.format("%Y-%m-%d");

  Ok(format!(
    "cs-{}-{}-{}-{}",
    head_date, remote_branch, remote_shorthash, head_shorthash,
  ))
}

fn get_original_cs_branch_name_for_cs_ref(
  cs_refname: &str,
) -> Result<String, GitError> {
  let branch_suffix = cs_refname.split("/cs-").last().expect("bad ref format");
  let meta_ref = format!("refs/heads/csmeta-{}", &branch_suffix);

  Ok(
    git::cs_git(&["cat-file", "blob", &format!("{}:commit.ref", meta_ref)])?
      .split("/")
      .last()
      .expect("bad ref format")
      .to_string(),
  )
}

fn get_original_cs_branch_name_for_commit(
  commitish: &str,
) -> Result<String, GitError> {
  let refname = git::cs_git(&[
    "for-each-ref",
    "--points-at",
    commitish,
    "--format=%(refname)",
    "refs/heads/cs-*",
  ])?;
  get_original_cs_branch_name_for_cs_ref(&refname)
}

pub fn get_branch_name() -> Result<String, GitError> {
  let head_branch = git::git(&["symbolic-ref", "HEAD"])?
    .split("/")
    .last()
    .expect("invalid ref name")
    .to_string();

  let head = git::git(&["rev-parse", "HEAD"])?;
  match get_original_cs_branch_name_for_commit(&head) {
    Ok(branch) => {
      if branch == head_branch {
        return Ok(head_branch);
      }
    }
    Err(_) => (),
  }

  let parent = git::git(&["rev-parse", "HEAD^"])?;
  match get_original_cs_branch_name_for_commit(&parent) {
    Ok(branch) => {
      if branch == head_branch {
        return Ok(head_branch);
      }
    }
    Err(_) => (),
  }

  create_branch_name()
}

pub fn get_meta_branch_name() -> Result<String, GitError> {
  let cs_branch = get_branch_name()?;
  Ok(format!("csmeta-{}", &cs_branch[3..]))
}
