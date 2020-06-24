/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use commitsync::git;

/** Print the upstream of the current repository.
 *
 * This is a debugging tool, not intended for regular use.
 */
pub fn main() -> commitsync::Result<()> {
  match git::get_upstream()? {
    Some(refname) => println!("{}", &refname),
    None => std::process::exit(1),
  }
  Ok(())
}
