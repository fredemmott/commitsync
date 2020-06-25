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
use std::str::FromStr;

mod printer;
pub use printer::Printer;

mod graph_printer;
pub use graph_printer::GraphPrinter;

pub enum Format {
  Short,
  Oneline,
}

impl FromStr for Format {
  type Err = String;
  fn from_str(data: &str) -> Result<Format, Self::Err> {
    match data {
      "oneline" => Ok(Format::Oneline),
      "short" => Ok(Format::Short),
      _ => Err("Invalid format".to_string()),
    }
  }
}

pub struct Options<'a> {
  pub fetch: bool,
  pub format: &'a Option<Format>,
}

fn printer_from_format(
  format: &Option<Format>,
) -> Result<Box<dyn Printer>, CSError> {
  match format {
    Some(Format::Oneline) => Ok(GraphPrinter::oneline()),
    Some(Format::Short) => Ok(GraphPrinter::short()),
    None => match git(&["config", "commitsync.format"]) {
      Err(_) => Ok(GraphPrinter::short()),
      Ok(s) => match Format::from_str(&s) {
        Err(_) => Err(CSError::UserError(format!(
          "Bad config option commitsync.format={}",
          &s,
        ))),
        Ok(f) => printer_from_format(&Some(f)),
      },
    },
  }
}

pub fn log(options: &Options) -> Result<(), CSError> {
  let printer_box = printer_from_format(&options.format)?;
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
