# Git CommitSync

Automatically sync your commits to a repository, with branch and
host tracking.

This is primarily intended to streamline working on multiple computers -
for example, short-lived EC2 nodes, or a Windows computer and a MacOS. It
can also be useful as part of a backup solution.

## Installation

1. Install Rust
2. Cargo build

## Usage

1. Create a repository to sync to; while it's possible to sync to your main
  repository, this isn't usually a good idea: CommitCloud will create a lot
  of branches, which are largely noise - and they'll automatically be public.
  It's usually best to create a private repository somewhere.
2. Configure passwordless access to this repository - for example, via ssh-agent.
4. Run `/path/to/commitsync init` (or `commitsync.exe`) to setup the repo
5. Run `git cs` to view synced commits

## Inspiration

This project is inspired by 'CommitCloud', part of the
[Eden](https://github.com/facebookexperimental/eden/) project.

## License

This code is distributed under the terms of the
[ISC License](LICENSE).  
I am providing code in the repository to you under an open source license.
Because this is my personal repository, the license you recieve to my code
is from me and not my employer (Facebook).
