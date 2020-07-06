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
3. Run `/path/to/commitsync init` (or `commitsync.exe`) to setup the repo
4. Run `git cs` to view synced commits
5. run `git config --global commitsync.format=oneline` for more consise output.

## Example

```
fred@Freds-MBP testrepo % git cs 
* a8e0545594f2d550079baba46fb0bb914625cf6d 
| Branch: fred@Freds-WindowsPC/bar 
| Author: Fred Emmott <fred@fredemmott.com> 
| Date:   Thu, 25 Jun 2020 17:55:48 -0700 
|  
| first commit on bar from windows PC 
|  
* 2b32bc54ae58f905b491b2372f81a8a4bdab30f7 
| Branch: fred@Freds-MBP/bar 
| Author: Fred Emmott <fred@fredemmott.com> 
| Date:   Wed, 24 Jun 2020 19:03:48 -0700 
|  
| initial commit on bar, from foo^ 
|  
| * a9b49f93905a9fb68495f307b1c6232078f1e24f 
| | Branch: fred@Freds-WindowsPC/foo 
| | Author: Fred Emmott <fred@fredemmott.com> 
| | Date:   Thu, 25 Jun 2020 17:55:02 -0700 
| |  
| | first commit on foo from windows PC 
| |  
| * 3962ac33a846bd9803827fbf71ab6af2434822bd 
|/  Branch: fred@Freds-MBP/foo 
|   Author: Fred Emmott <fred@fredemmott.com> 
|   Date:   Wed, 24 Jun 2020 19:03:29 -0700 
|    
|   second commit on foo 
|    
* fd55ffd0ca58ae08fb9be813b381e7e39e0d933d 
  Author: Fred Emmott <fred@fredemmott.com> 
  Date:   Wed, 24 Jun 2020 19:03:23 -0700 
   
  initial commit on foo 
fred@Freds-MBP testrepo % git cs --format=oneline 
* (fred@Freds-WindowsPC/bar) a8e0545 first commit on bar from windows PC 
|  
* (fred@Freds-MBP/bar) 2b32bc5 initial commit on bar, from foo^ 
|  
| * (fred@Freds-WindowsPC/foo) a9b49f9 first commit on foo from windows PC 
| |  
| * (fred@Freds-MBP/foo) 3962ac3 second commit on foo 
|/   
|    
* fd55ffd initial commit on foo 
fred@Freds-MBP testrepo % git checkout -b foo-from-win a9b49f9 
Switched to a new branch 'foo-from-win' 
fred@Freds-MBP testrepo % git log -1 
commit a9b49f93905a9fb68495f307b1c6232078f1e24f (HEAD -> foo-from-win, mybranch, fromother-pc) 
Author: Fred Emmott <fred@fredemmott.com> 
Date:   Thu Jun 25 17:55:02 2020 -0700 
 
    first commit on foo from windows PC 
fred@Freds-MBP testrepo %
```

## Inspiration

This project is inspired by 'CommitCloud', part of the
[Eden](https://github.com/facebookexperimental/eden/) project.

## License

This code is distributed under the terms of the
[ISC License](LICENSE).  
I am providing code in the repository to you under an open source license.
Because this is my personal repository, the license you recieve to my code
is from me and not my employer (Facebook).
