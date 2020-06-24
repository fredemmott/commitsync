/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

#[derive(Debug)]
pub struct FailedCommandOutput {
  pub command: String,
  pub stdout: String,
  pub stderr: String,
}

#[derive(Debug)]
pub enum Error {
  SpawnError(subprocess::PopenError),
  NonZeroExit(FailedCommandOutput),
}

impl From<subprocess::PopenError> for Error {
  fn from(error: subprocess::PopenError) -> Error {
    Error::SpawnError(error)
  }
}
