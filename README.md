## nit - A git CLI wrapper to make your git better

![cover](https://i.imgur.com/GuMKIgz.png)

A CLI to replace your `git` command, so your git workflow can become more consistent, and fun. Also, commit message can partially follow [the Conventional Changelog](https://github.com/conventional-changelog/conventional-changelog) ecosystem. And yes, it is build on top of [Rust](https://www.rust-lang.org/)

## Install From Source

Assuming you have installed [Git](https://github.com/git-guides/install-git) and [Rust](https://www.rust-lang.org/tools/install) in your system, you can clone the repository and build it from source:
  
```bash
git clone https://github.com/hisamafahri/nit
cd nit
cargo build
```

Your executable should be available in the `target/debug` directory.

## Usage

### Commit Changes

To commit staged changes:

```bash
nit commit # or: nit c
```

To commit all of the changes in the current directory, you can easily run:

```bash
nit commit --all # or: nit c -a

# This command will substitute:
# git add .
# nit commit
```

By running it, you will add all of the changes in the *current directory* and commit it automatically. :)

## Pushing Changes

```bash
nit push # or: nit p
```

This command will push your ***current branch*** into your remote repo. If there are multiple remote repo aliases, it will prompt you to choose:

```bash
$ nit p

? Where you want to push your changes? >
> remote1: https://github.com/hisamafahri/remote1
  remote2: https://github.com/hisamafahri/remote2
  remote3: https://github.com/hisamafahri/remote3
```

## Author

[Hisam A Fahri](https://hisamafahri.com): [@hisamafahri](https://github.com/hisamafahri)

## License

[MIT](LICENSE)