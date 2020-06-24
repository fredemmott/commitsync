/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

pub mod branch_names;
pub mod git;

pub use git::GitError;

mod store_commit;
pub use store_commit::store_commit;
mod init_repo;
pub use init_repo::init_repo;
mod meta_branch_info;
pub use meta_branch_info::*;

#[derive(Debug)]
pub enum CSError {
  GitError(GitError),
  UserError(String),
}

impl std::fmt::Display for CSError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self)
  }
}
impl std::error::Error for CSError {}

impl From<GitError> for CSError {
  fn from(error: GitError) -> CSError {
    CSError::GitError(error)
  }
}

pub type FIXMEResult<T> = std::result::Result<T, CSError>;
