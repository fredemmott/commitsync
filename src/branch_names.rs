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
