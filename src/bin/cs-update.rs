/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

/** Store the current ref in CommitSync.
 *
 * This is a debugging tool, not intended for regular use.
 */
pub fn main() -> commitsync::Result<()> {
  let (cs_ref, cs_meta_ref) = commitsync::store_commit()?;
  println!("{}\n{}", &cs_ref, cs_meta_ref);
  Ok(())
}
