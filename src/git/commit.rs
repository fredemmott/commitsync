/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use crate::git::exec::*;
use crate::git::raw_commit::*;
use crate::*;
use chrono::prelude::*;

#[derive(Debug)]
pub struct Commit {
  pub committed_at: DateTime<FixedOffset>,
}

pub fn get_real_commit(commitish: &str) -> Result<Commit, GitError> {
  let blob = git(&["cat-file", "commit", commitish])?;
  Ok(parse_commit_blob(&blob).expect("parse commit"))
}

pub fn get_cs_commit(commitish: &str) -> Result<Commit, GitError> {
  let blob = cs_git(&["cat-file", "commit", commitish])?;
  Ok(parse_commit_blob(&blob).expect("parse commit"))
}

pub fn parse_commit_blob(data: &str) -> Option<Commit> {
  let raw = parse_raw_commit(data)?;
  let committer = &raw.fields["committer"];
  let mut words: Vec<&str> = committer.split(" ").collect();
  words.reverse();
  let (commit_tz, commit_timestamp) = (
    words[0].parse::<i32>().unwrap(),
    words[1].parse::<i64>().unwrap(),
  );
  let tz_hours = commit_tz / 100;
  let tz_minutes = (tz_hours * 60) + (commit_tz % 100);
  let tz_seconds = tz_minutes * 60;
  Some(Commit {
    committed_at: DateTime::<FixedOffset>::from_utc(
      NaiveDateTime::from_timestamp(commit_timestamp, 0),
      FixedOffset::east(tz_seconds),
    ),
  })
}
