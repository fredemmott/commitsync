/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */
use crate::*;
use std::collections::HashMap;

pub trait Printer {
  fn progress(&self, message: &str) -> ();
  fn print_metas(&self, metas: &HashMap<String, BranchMetadata>) -> Result<(), CSError>;
}
