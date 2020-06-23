/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use commitsync::git;
use commitsync::git::git;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
  /// Just separate fields and message, don't parse the fields
  #[structopt(long)]
  raw: bool,

  /// What revision to print
  commitish: String,
}

/** Parse a git commit.
 *
 * This is a debugging tool, not intended for regular use.
 */
pub fn main() -> () {
  let cli = Cli::from_args();
  let sha = git(&["rev-parse", &cli.commitish]).unwrap();
  let blob = git(&["cat-file", "commit", &sha]).unwrap();

  if cli.raw {
    let commit = git::raw_commit::parse_raw_commit(&blob).unwrap();
    println!("{:#?}", commit);
    return;
  }

  let commit = git::parse_commit_blob(&blob).unwrap();
  println!("{:#?}", commit);
  return;
}
