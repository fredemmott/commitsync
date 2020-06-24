/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

pub mod branch_names;
pub mod git;

mod store_commit;
pub use store_commit::store_commit;
mod init_repo;
pub use init_repo::init_repo;

#[derive(Debug)]
pub enum Error {
  Git(subprocess::PopenError),
  UserError(String),
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self)
  }
}
impl std::error::Error for Error {}

impl From<subprocess::PopenError> for Error {
  fn from(error: subprocess::PopenError) -> Error {
    Error::Git(error)
  }
}

pub type Result<T> = std::result::Result<T, Error>;
