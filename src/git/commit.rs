/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use crate::git::git;
use crate::git::raw_commit::*;
use chrono::prelude::*;

#[derive(Debug)]
pub struct Commit {
  pub committed_at: DateTime<FixedOffset>,
}

pub fn get_commit(commitish: &str) -> Option<Commit> {
  let blob = git(&["cat-file", "commit", commitish])?;
  parse_commit_blob(&blob)
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
