/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use commitsync::{branch_names::*, *};

/** Print the branch name that CommitSync would generated if needed.
 *
 * This is that branch name that will be used if CommitSync is unable to use
 * a previous branch.
 *
 * This is a debugging tool, not intended for regular use.
 */
pub fn main() -> Result<(), CSError> {
  println!(
    "Data: {}\nMeta: {}",
    create_branch_name()?,
    create_meta_branch_name()?,
  );
  Ok(())
}
