/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use commitsync::{branch_names::*, *};

/** Print the branch name that CommitSync would use to sync.
 *
 * This is a debugging tool, not intended for regular use.
 */
pub fn main() -> Result<(), CSError> {
  println!("{}", &get_branch_name()?);
  Ok(())
}
