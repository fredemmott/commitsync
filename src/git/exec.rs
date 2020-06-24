/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use subprocess::{Exec, Redirection};

use crate::{git::dirs::cs_git_dir, git::*};

fn trace(cmd: &str, args: &[&str]) -> () {
  match std::env::var_os("COMMITSYNC_TRACE")
    .unwrap_or(std::ffi::OsString::new())
    .as_os_str()
    .to_str()
    .unwrap_or("")
  {
    "1" | "true" | "on" | "yes" => eprintln!("{}\t{:?}", cmd, args),
    _ => (),
  }
}

fn exec_git(cmd: subprocess::Exec) -> Result<String, GitError> {
  let cmdline = cmd.to_cmdline_lossy();
  let out = cmd
    .stdout(Redirection::Pipe)
    .stderr(Redirection::Pipe)
    .capture()?;
  if out.success() {
    Ok(out.stdout_str().trim().to_string())
  } else {
    Err(GitError::NonZeroExit(FailedCommandOutput {
      command: cmdline,
      stdout: out.stdout_str().trim().to_string(),
      stderr: out.stderr_str().trim().to_string(),
    }))
  }
}

/// Execute `git` in the real repository.
pub fn git(args: &[impl AsRef<str>]) -> Result<String, GitError> {
  let args : Vec<&str> = args.iter().map(AsRef::as_ref).collect();
  trace("git", &args);
  let cmd = Exec::cmd("git").args(&args);
  exec_git(cmd)
}

/// Execute `git` in the CommitSync repository.
pub fn cs_git(args: &[impl AsRef<str>]) -> Result<String, GitError> {
  let args : Vec<&str> = args.iter().map(AsRef::as_ref).collect();
  trace("cs_git", &args);
  let git_dir = cs_git_dir()?;
  let mut index_file = git_dir.clone();
  index_file.push("index");

  let cmd = Exec::cmd("git")
    .args(&args)
    .env("GIT_DIR", &git_dir)
    .env("GIT_INDEX_FILE", &index_file);
  exec_git(cmd)
}
