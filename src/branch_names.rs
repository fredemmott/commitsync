/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use crate::git;

pub fn create_branch_name() -> Option<String> {
  let remote_ref = git::get_upstream()?;
  let remote_branch = remote_ref.split("/").last()?;
  let remote_shorthash = git::git(&["rev-parse", "--short", &remote_ref])?;

  let head_shorthash = git::git(&["rev-parse", "--short", "HEAD"])?;
  let head_info = git::get_commit(&head_shorthash)?;
  let head_date = head_info.committed_at.format("%Y-%m-%d");

  Some(format!(
    "cs-{}-{}-{}-{}",
    head_date, remote_branch, remote_shorthash, head_shorthash,
  ))
}

fn get_original_cs_branch_name_for_cs_ref(cs_refname: &str) -> Option<String> {
  let branch = cs_refname.split("/").last()?;
  let meta_ref = format!("refs/heads/csmeta-{}", branch);

  Some(
    git::cs_git(&["cat-file", "blob", &format!("{}:commit.ref", meta_ref)])?
      .split("/")
      .last()?
      .to_string(),
  )
}

fn get_original_cs_branch_name_for_commit(commitish: &str) -> Option<String> {
  let refname = git::cs_git(&[
    "for-each-ref",
    "--points-at",
    commitish,
    "--format=%(refname)",
    "refs/heads/cs-*",
  ])?;
  get_original_cs_branch_name_for_cs_ref(&refname)
}

pub fn get_branch_name() -> Option<String> {
  let head_branch = git::git(&["symbolic-ref", "HEAD"])?
    .split("/")
    .last()?
    .to_string();

  let head = git::git(&["rev-parse", "HEAD"])?;
  match get_original_cs_branch_name_for_commit(&head) {
    Some(branch) => {
      if branch == head_branch {
        return Some(head_branch);
      }
    }
    None => (),
  }

  let parent = git::git(&["rev-parse", "HEAD^"])?;
  match get_original_cs_branch_name_for_commit(&parent) {
    Some(branch) => {
      if branch == head_branch {
        return Some(head_branch);
      }
    }
    None => (),
  }

  create_branch_name()
}
