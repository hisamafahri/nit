## nit - A git CLI wrapper to make your git better

---

![cover](https://i.imgur.com/GuMKIgz.png)

[![Build](https://github.com/hisamafahri/nit/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/hisamafahri/nit/actions/workflows/build.yml)
[![Release](https://github.com/hisamafahri/nit/actions/workflows/release.yml/badge.svg?branch=v0.1.1)](https://github.com/hisamafahri/nit/actions/workflows/release.yml)

A CLI to replace your `git` command, so your git workflow can become more consistent, and fun. Also, commit message can partially follow [the Conventional Changelog](https://github.com/conventional-changelog/conventional-changelog) ecosystem. And yes, it is build on top of [Rust](https://www.rust-lang.org/)

## Requirements

Since this CLI is wrapping the existing [git](https://github.com/git-guides/install-git) CLI, git is required.


## Installation

- From binaries
  
  You can download the latest version of this CLI from the [Release Page](https://github.com/hisamafahri/nit/releases).

- From Source
  
  Assuming you have installed [Rust](https://www.rust-lang.org/tools/install) in your system, you can clone the repository and build it from source:
  
  ```bash
  git clone https://github.com/hisamafahri/nit
  cd nit
  cargo build
  ```
  
  Your executable should be available in the `target/debug` directory.

## Usage

- Staging changes

  ```bash
  # stage specific file
  nit add # or: nit a

  # stage all files
  nit add --all # or: nit a -a
  ```

- Commit changes

  To commit changes:

  ```bash
  # commit staged changes
  nit commit # or: nit c

  # commit all files (staged and unstaged)
  nit commit --all # or: nit c -a

  # This command will substitute:
  # nit add --all
  # nit commit
  ```

- Pushing Changes

  ```bash
  nit push # or: nit p
  ```

## Author

[Hisam A Fahri](https://hisamafahri.com): [@hisamafahri](https://github.com/hisamafahri)

## License

[MIT](LICENSE)