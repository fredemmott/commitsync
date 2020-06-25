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
use std::collections::HashMap;

mod printer;
pub use printer::Printer;

mod graph_printer;
pub use graph_printer::GraphPrinter;

pub struct Options {
  pub fetch: bool,
  pub printer: Option<Box<dyn Printer>>,
}

pub fn log(options: Options) -> Result<(), CSError> {
  let printer_box = match options.printer {
    Some(printer) => printer,
    None => match git(&["config", "commitsync.format"])
      .unwrap_or(String::new())
      .as_ref()
    {
      "oneline" => GraphPrinter::oneline(),
      "short" | "" => GraphPrinter::short(),
      value => {
        return Err(CSError::UserError(format!(
          "Configuration option 'commitsync.format={}' is invalid",
          value
        )))
      }
    },
  };
  let printer = &*printer_box;

  if options.fetch {
    fetch(printer);
  }

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
    .map(|name| {
      (
        name.split("/csmeta-").last().unwrap().to_string(),
        meta_branch_info(name).expect("Failed to load meta branch"),
      )
    })
    .unique_by(|(_name, meta)| meta.commit_sha.to_string())
    .collect::<HashMap<String, BranchMetadata>>();

  printer.print_metas(&metas)
}

fn fetch(printer: &dyn Printer) -> () {
  printer.progress("Fetching...");
  match cs_git(&["fetch", "commitsync"]) {
    Ok(_) => (),
    Err(_) => eprintln!("  {}", "Fetching failed, showing stale data".red()),
  }
  printer.progress("\r");
}
