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
use std::io::prelude::*;

fn fetch() -> () {
  print!("Fetching...");
  let _ = std::io::stdout().flush();
  match cs_git(&["fetch", "commitsync"]) {
    Ok(_) => (),
    Err(_) => eprintln!("  {}", "Fetching failed, showing stale data".red()),
  }
  print!("\r");
  let _ = std::io::stdout().flush();
}

pub fn list() -> Result<(), CSError> {
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

  let metas: Vec<(String, BranchMetadata)> = refs
    .split("\n")
    .map(|name| {
      (
        name.split("/csmeta-").last().unwrap().to_string(),
        meta_branch_info(name).expect("Failed to load meta branch"),
      )
    })
    .unique_by(|(_name, meta)| meta.commit_sha.to_string())
    .collect();

  let start_tag = "<<<CS_REFS(";
  let end_tag = ")CS_REFS>>>";
  let mut args: Vec<String> = vec!["log".to_string(), "--graph".to_string()];
  args.push(format!(
    "--format={}\n{}%D{}\nAuthor: %aN <%aE>\nDate:   %aD\n\n%s\n",
    "commit %H".yellow(),
    &start_tag,
    &end_tag
  ));
  for (_, meta) in &metas {
    args.push(format!("{}..{}", &meta.upstream_sha, &meta.commit_sha))
  }
  let log = cs_git(&args)?;
  let metas: std::collections::HashMap<_, _> = metas.into_iter().collect();

  for line in log.lines() {
    match line.find(end_tag) {
      None => println!("{}", line),
      Some(end_tag_pos) => {
        let start_tag_pos = line.find(start_tag).expect("a start tag");
        if end_tag_pos == start_tag_pos + start_tag.len() {
          continue;
        }

        let keys = line[(start_tag_pos + start_tag.len())..end_tag_pos]
          .split(", ")
          .map(|ref_name| {
            if &ref_name[0..3] == "cs-" {
              &ref_name[3..]
            } else {
              ref_name.split("/cs-").last().unwrap()
            }
          })
          .unique();
        let prefix = &line[..start_tag_pos];
        let prefix = match &prefix.rfind("/") {
          None => prefix.to_string(),
          Some(idx) => format!("{} {}", &prefix[0..*idx], &prefix[idx + 1..]),
        };
        let mut first: bool = true;
        for key in keys {
          let meta = &metas[key];
          println!(
            "{}Branch: {}@{}/{}",
            if first {
              &line[..start_tag_pos]
            } else {
              &prefix
            },
            &meta.user,
            &meta.hostname.split(".").nth(0).unwrap().cyan(),
            &meta
              .commit_ref
              .split("/")
              .last()
              .expect("a valid ref")
              .green(),
          );
          first = false;
        }
      }
    }
  }
  Ok(())
}
