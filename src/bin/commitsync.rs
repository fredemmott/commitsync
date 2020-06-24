/*
 * Copyright (c) 2020-present, Fred Emmott <fred@fredemmott.com>
 * All rights reserved.
 *
 * This source code is licensed under the ISC license found in the LICENSE file
 * in the root directory of this source tree.
 */

use structopt::StructOpt;

enum Command {
  Init,
  PostCommit,
}

impl std::str::FromStr for Command {
  type Err = String;
  fn from_str(data: &str) -> Result<Command, Self::Err> {
    match data {
      "init" => Ok(Command::Init),
      "post-commit" => Ok(Command::PostCommit),
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
      commitsync::init_repo().unwrap();
    }
    Command::PostCommit => {
      commitsync::store_commit().unwrap();
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
        println!("TODO: list mode");
      } else {
        run(&Command::Init, &cli)
      }
    }
  }
}
