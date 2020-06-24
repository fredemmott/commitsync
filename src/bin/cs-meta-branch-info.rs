/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
  /// Which meta branch to dump
  meta_ref: String,
}

/** Dump the information in a CommitSync metadata branch.
 *
 * This is a debugging tool, not intended for regular use.
 */
pub fn main() -> () {
  let cli = Cli::from_args();
  let info =
    commitsync::meta_branch_info(&cli.meta_ref).expect("couldn't parse branch");
  println!("{:#?}", info);
  return;
}
