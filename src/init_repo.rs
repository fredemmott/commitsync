/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use crate::git::dirs::*;
use crate::git::*;
use crate::Result;
use colored::*;
use std::fs::File;
use std::io::prelude::*;

fn create_repo() -> Result<()> {
  println!("Creating local repo...");
  let git_dir = real_git_dir()?.into_os_string().into_string().unwrap();
  let cs_git_dir = cs_git_dir()?.into_os_string().into_string().unwrap();
  git(&["init", "--bare", &cs_git_dir])?;

  let mut file =
    File::create(&format!("{}/objects/info/alternates", &cs_git_dir)).unwrap();
  file
    .write_all(&format!("{}/objects", &git_dir).into_bytes())
    .unwrap();
  let mut file =
    File::create(&format!("{}/objects/info/alternates", &git_dir)).unwrap();
  file
    .write_all(&format!("{}/objects", &cs_git_dir).into_bytes())
    .unwrap();

  Ok(())
}

fn add_alias() -> Result<()> {
  let global_alias = git(&["config", "alias.cs"]).unwrap_or(String::new());
  if global_alias != "" {
    return Ok(());
  }

  println!("{}", "Where would like `git cs` to work?".bold());
  println!(" - {}: on the system", "global".green());
  println!("   - useful if you want to use CommitSync with multiple repos");
  println!("   - useful if you use containers (e.g. WSL, Docker)");
  println!(" - {}: just this repo", "local".green());
  // INCOMPLETE
  Ok(())
}

pub fn init_repo() -> Result<()> {
  if cs_git_dir()?.exists() {
    return Err(crate::Error::UserError(format!(
      "{} already exists, aborting.",
      &cs_git_dir()?.into_os_string().into_string().unwrap()
    )));
  }
  create_repo()?;
  add_alias()?;

  Ok(())
}
