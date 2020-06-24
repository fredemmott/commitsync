/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use crate::git::dirs::*;
use crate::git::*;
use crate::*;
use colored::*;
use std::fs::File;
use std::io::prelude::*;

fn create_repo() -> Result<(), CSError> {
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

fn add_alias() -> Result<(), CSError> {
  let global_alias = git(&["config", "alias.cs"]).unwrap_or(String::new());
  if global_alias != "" {
    return Ok(());
  }

  println!("Where would like `git cs` to work?");
  println!(" - {}: every repository", "global".green());
  println!("   - useful if you want to use CommitSync with multiple repos");
  println!("   - useful if you use containers (e.g. WSL, Docker)");
  println!("   - allows `git cs init` in other repositories");
  println!(" - {}: just this repository", "local".green());
  print!("{} [global] {} ", "Your choice?".bold(), ">".bold());
  let _ = std::io::stdout().flush();

  let mut buf = String::new();
  let exe = std::env::current_exe()
    .expect("should have a current exe")
    .into_os_string()
    .into_string()
    .expect("Invalid UTF8")
    .replace("\\", "/");
  let alias = format!("!{}", exe);
  loop {
    std::io::stdin()
      .read_line(&mut buf)
      .expect("Failed to read a line?");
    match &buf.trim()[..] {
      "" | "global" => {
        git(&["config", "--global", "alias.cs", &alias])?;
        return Ok(());
      }
      "local" => {
        git(&["config", "alias.cs", &alias])?;
        return Ok(());
      }
      _ => eprintln!("Enter 'local' or 'global'"),
    }
  }
}

fn configure_remote() -> Result<(), CSError> {
  println!(
    "Where should CommitSync push?

All branches will be public; you probably don't want this to be your
main upstream repository.

Example: git@example.com:commitsync/myrepo.git"
  );
  let _ = std::io::stdout().flush();

  let mut url = String::new();
  loop {
    print!("{} ", "Remote URL>".bold());
    std::io::stdout().flush().unwrap();
    std::io::stdin()
      .read_line(&mut url)
      .expect("Failed to read a line?");
    url = url.trim().to_string();
    match &url[..] {
      "" => (),
      _ => break,
    }
  }
  cs_git(&["remote", "add", "commitsync", &url])?;
  Ok(())
}

fn fetch_data() -> Result<(), CSError> {
  println!("Fetching remote CommitSync data...");
  cs_git(&["fetch"])?;
  println!("...done!");
  Ok(())
}

#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;
#[cfg(target_family = "unix")]
fn make_executable(path: &std::path::PathBuf) -> () {
  let mut perms = std::fs::metadata(&path)
    .expect("retrieving permissions")
    .permissions();
  perms.set_mode(0o755);
  std::fs::set_permissions(&path, perms)
    .expect("Failed to chmod 755 the commit hook");
}

#[cfg(target_family = "windows")]
fn make_executable(_path: &std::path::PathBuf) -> () {
  // nothing to do
}

fn setup_hook() -> Result<(), CSError> {
  let line = "git cs post-commit";
  let mut path = real_git_dir()?;
  path.push("hooks");
  path.push("post-commit");
  if path.exists() {
    let content = std::fs::read_to_string(&path).unwrap();
    if !content.contains(line) {
      println!(
        "{}\nTo finish installation, add '{}' to {}",
        "INSTALLING THE HOOK".bold(),
        &line,
        &path
          .into_os_string()
          .into_string()
          .expect("Invalid UTF8 path"),
      )
    }
    return Ok(());
  }

  std::fs::write(&path, &format!("#!/bin/sh\n{}\n", &line))
    .expect("Failed to write hook");
  make_executable(&path);
  Ok(())
}

pub fn init_repo() -> Result<(), CSError> {
  if cs_git_dir()?.exists() {
    return Err(CSError::UserError(format!(
      "{} already exists, aborting.",
      &cs_git_dir()?.into_os_string().into_string().unwrap()
    )));
  }
  create_repo()?;
  add_alias()?;
  configure_remote()?;
  fetch_data()?;
  setup_hook()?;

  println!("{} Run `git cs` to access CommitSync.", "All done!".bold());

  Ok(())
}
