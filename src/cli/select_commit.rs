/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use crate::{git::*, *};
use colored::*;
use itertools::Itertools;

fn fetch() -> () {
  println!("Fetching...");
  match cs_git(&["fetch", "commitsync"]) {
    Ok(_) => (),
    Err(_) => eprintln!("  {}", "Fetching failed, showing stale data".red()),
  }
}

pub fn print_meta(n: u16, meta: &BranchMetadata) -> Result<(), GitError> {
  let (cols, _) = term_size::dimensions().unwrap_or((80, 0));

  let date = format!("{}", meta.meta_committed_at.format("%Y-%m-%d %X"));
  println!(
    "{}{}\r{} Branch {} {}",
    " ".repeat(cols - date.len()),
    date.yellow().bold(),
    format!("{}.", n).bold(),
    meta
      .commit_ref
      .split("/")
      .last()
      .expect("valid ref")
      .bold()
      .green(),
    format!(
      "({}@{})",
      &meta.user,
      &meta
        .hostname
        .split(".")
        .nth(0)
        .expect("expected a hostname")
    )
    .cyan(),
  );

  // Just printing to the user, so using porcelain is fine
  let log = git(&[
    "log",
    "--graph",
    &format!("--format={}%<|({},trunc) %s", "%h".yellow(), cols - 2),
    &format!("{}..{}", &meta.upstream_sha, &meta.commit_sha),
  ])?;
  for line in log.lines() {
    println!("  {}", line);
  }
  Ok(())
}

pub fn select_commit() -> Result<(), CSError> {
  fetch();

  let refs = cs_git(&[
    "for-each-ref",
    "--format=%(refname)",
    "--sort=committerdate",
    "refs/heads/csmeta-*",
    "refs/remotes/commitsync/csmeta-*",
  ])
  .unwrap_or(String::new());
  if refs.is_empty() {
    println!(
      "  {} - {}",
      "No commits yet".red(),
      "Welcome to CommitSync!".bold()
    );
    return Ok(());
  }

  let metas = refs
    .split("\n")
    .map(|name| meta_branch_info(name).expect("Failed to load meta branch"))
    .unique_by(|info| info.commit_sha.to_string());

  let mut i: u16 = 0;
  for meta in metas {
    i += 1;
    print_meta(i, &meta)?;
  }
  Ok(())
}
