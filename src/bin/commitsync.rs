/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use colored::*;
use std::io::prelude::*;
use structopt::StructOpt;

enum Command {
  Init,
  PostCommit,
  List,
}

impl std::str::FromStr for Command {
  type Err = String;
  fn from_str(data: &str) -> Result<Command, Self::Err> {
    match data {
      "init" => Ok(Command::Init),
      "post-commit" => Ok(Command::PostCommit),
      "list" => Ok(Command::List),
      _ => Err("Invalid command".to_string()),
    }
  }
}

#[derive(StructOpt)]
struct Cli {
  command: Option<Command>,
}

fn run(command: &Command, _cli: &Cli) -> () {
  match command {
    Command::Init => {
      commitsync::cli::init_repo().unwrap();
    }
    Command::PostCommit => {
      commitsync::store_commit().unwrap();
    }
    Command::List => {
      commitsync::cli::list().unwrap();
    }
  }
}

fn main() -> () {
  let cli = Cli::from_args();
  match &cli.command {
    Some(it) => run(&it, &cli),
    None => {
      let dir = commitsync::git::dirs::cs_git_dir().unwrap();
      if dir.exists() {
        run(&Command::List, &cli)
      } else {
        print!(
          "{}",
          "Would you like to initialize CommitSync? y/n [y]> ".bold()
        );
        std::io::stdout().flush().unwrap();
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        match buf.trim() {
          "" | "y" | "yes" => run(&Command::Init, &cli),
          _ => (),
        }
      }
    }
  }
}
