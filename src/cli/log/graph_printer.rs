/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use crate::{cli::log::Printer, git::*, *};
use colored::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::prelude::*;

const START_TAG: &'static str = "<<<CS_REFS(";
const END_TAG: &'static str = ")CS_REFS>>>";

pub struct GraphPrinter {
  format: String,
}

impl Printer for GraphPrinter {
  fn progress(&self, message: &str) -> () {
    print!("{}", message);
    let _ = std::io::stdout().flush();
  }

  fn print_metas(
    &self,
    metas: &HashMap<String, BranchMetadata>,
  ) -> Result<(), CSError> {
    let mut args: Vec<String> = vec![
      "log".to_string(),
      "--graph".to_string(),
      self.format.to_string(),
    ];
    for (_, meta) in metas.iter() {
      args.push(format!("{}..{}", &meta.upstream_sha, &meta.commit_sha))
    }
    let log = cs_git(&args)?;
    let mut lines = log.lines();

    while let Some(line) = lines.next() {
      match line.find(END_TAG) {
        None => println!("{}", line),
        Some(end_tag_pos) => {
          let start_tag_pos = line.find(START_TAG).expect("a start tag");
          let trailing = &line[end_tag_pos + END_TAG.len()..];
          if end_tag_pos == start_tag_pos + START_TAG.len() {
            let prefix = &line[..start_tag_pos];
            if trailing.is_empty() {
              // Keep any drawing characters: if this is the first commit in a
              // branch, there's a `|/ ` that we want to preserve
              let next = match lines.next() {
                Some(next) => next,
                None => continue,
              };
              println!("{}{}\r{}", &prefix, &next[start_tag_pos..], &prefix);
            } else {
              println!("{}{}", &prefix, &trailing);
            }
            continue;
          }

          let keys = line[(start_tag_pos + START_TAG.len())..end_tag_pos]
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
          let pretty_refs = keys
            .map(|key| {
              let meta = &metas[key];
              format!(
                "{}@{}/{}",
                &meta.user,
                &meta.hostname.split(".").nth(0).unwrap().cyan(),
                &meta
                  .commit_ref
                  .split("/")
                  .last()
                  .expect("a valid ref")
                  .green(),
              )
            })
            .collect::<Vec<String>>();
          if trailing.is_empty() {
            let prefix = match &prefix.rfind("/") {
              None => prefix.to_string(),
              Some(idx) => {
                format!("{} {}", &prefix[0..*idx], &prefix[idx + 1..])
              }
            };

            let mut first: bool = true;
            for pretty_ref in pretty_refs {
              println!(
                "{}Branch: {}",
                if first {
                  &line[..start_tag_pos]
                } else {
                  &prefix
                },
                &pretty_ref,
              );
              first = false;
            }
          } else {
            println!(
              "{}{}{}{} {}",
              &prefix,
              "(".yellow(),
              &pretty_refs.join(", "),
              ")".yellow(),
              &trailing
            );
          }
        }
      }
    }
    Ok(())
  }
}

impl GraphPrinter {
  pub fn oneline() -> Box<dyn Printer> {
    Box::new(GraphPrinter {
      format: format!(
        "--format={}%D{}{} %s\n",
        START_TAG,
        END_TAG,
        "%h".yellow(),
      ),
    })
  }

  pub fn short() -> Box<dyn Printer> {
    Box::new(GraphPrinter {
      format: format!(
        "--format={}\n{}%D{}\nAuthor: %aN <%aE>\nDate:   %aD\n\n%s\n",
        "%H".yellow(),
        START_TAG,
        END_TAG,
      ),
    })
  }
}
