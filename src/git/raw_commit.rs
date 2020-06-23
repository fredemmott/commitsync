/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */
use std::collections::HashMap;

#[derive(Debug)]
pub struct RawCommit<'a> {
  pub fields: HashMap<&'a str, String>,
  pub message: &'a str,
}

pub fn parse_raw_commit(data: &str) -> Option<RawCommit> {
  parse_raw_commit_impl(data, Vec::new())
}

// A value is continued on subsequent lines if those lines start with a space
fn parse_value_continuation<'a>(
  data: &'a str,
  acc: &str,
) -> Option<(String, &'a str)> {
  if data.len() == 0 || !data.starts_with(" ") {
    return Some((acc.to_string(), data));
  }

  let eol = data.find("\n")?;
  let acc = [acc, &data[1..eol]].join("\n");
  parse_value_continuation(&data[(eol + 1)..], &acc)
}

fn parse_raw_commit_impl<'a>(
  data: &'a str,
  acc: Vec<(&'a str, String)>,
) -> Option<RawCommit<'a>> {
  let eol = data.find("\n")?;

  if eol == 0 {
    return Some(RawCommit {
      message: &data[1..],
      fields: acc.into_iter().map(|(key, value)| (key, value)).collect(),
    });
  }

  let separator_idx = data.find(" ")?;
  let key = &data[0..separator_idx];
  let value_first_line = &data[(separator_idx + 1)..eol];
  let (value, rest) =
    parse_value_continuation(&data[(eol + 1)..], value_first_line)?;
  let mut acc = acc.clone();
  acc.push((key, value));

  parse_raw_commit_impl(rest, acc)
}
