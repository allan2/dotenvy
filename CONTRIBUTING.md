# Contributing to _dotenvy_

**Thank you very much for considering to contribute to this project!**

We welcome any form of contribution, including:

- reporting or fixing bugs
- requesting or adding features
- doc changes (including spelling/grammar corrections)
- adding examples
- asking for help, sharing ideas

**Note**: Before making a large change, it is a good idea to open an issue
describing the change to solicit feedback and guidance. This will increase the
likelihood of the PR getting merged and help avoid multiple people writing the
same solution.

This guide will help you get started. Do not let this guide intimidate you. It
should be considered a map to help you navigate the process.

## Conduct

The `dotenvy` project adheres to the [Rust Code of
Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## Issues

Please feel free to open new [issues](https://github.com/allan2/dotenvy/issues),
or comment on existing ones. If you would like to attempt
to implement a change, you can ask for help or guidance on solving a problem, or
just let us know you are working on it.

A maintainer will mark the issue as assigned to you to avoid other people
unknowingly working on the same thing. If you would like to work on an assigned issue that has not been updated in a while, just ask to have it reassigned to you.

This project sometimes has longer form
[discussions](https://github.com/allan2/dotenvy/discussions). Feel free to
comment on existing threads or open your own.

## Pull Requests

Pull Requests are the way concrete changes are made to the code, documentation,
and dependencies in the dotenvy repository.

Even tiny PRs, like a one character change to fix a typo in the docs, are greatly appreciated. Before making a large change, it is recommended to first open an issue describing the change as to solicit
feedback and guidance. This will increase the likelihood of the PR getting
merged.

### Tests

If you are modifying the code, make sure all tests pass. If you are adding new functionality, please add related tests.

### Discuss and update 

If you receive feedback on your PR, please don't be discouraged. It's just part of the process to ensure that changes to the project are correct and necessary.

Any community member can review a PR.

#### Commit Squashing

There is no need to squash your commits manually.

### CI

Please ensure that your PR passes the CI checks:

- all tests pass on stable and [MSRV]
- no [clippy](https://github.com/rust-lang/rust-clippy) warnings or errors
- formatted with [Rustfmt](https://github.com/rust-lang/rustfmt)
- [Rustdoc](https://doc.rust-lang.org/rustdoc/write-documentation/linking-to-items-by-name.html)
  links work

The [`ci-check.sh`](ci-check.sh) can help with this.

#### Add an entry to the changelog

If your contribution changes the behavior of dotenvy, please update the
[`CHANGELOG.md`](CHANGELOG.md) file and describe your changes. This makes the
release process much easier and therefore helps to get your changes into a new
dotenvy release faster.

The top of the `CHANGELOG` contains an **Unreleased** section with a few
subsections (Added, Changed, etc) Please add your entry to the subsection
that best describes your change. If a relevant subsection does not yet exist, please
create it.

Entries follow this format:

```md
### Changed

- Short description of what has been changed ([PR #123](pull.request.url)) by [username](github.profile.url)
- [**BREAKING**] Please prefix any breaking changes
```

Here, `#123` is the number of your pull request.

**NOTE**: It is a good idea to push the
changelog update to your branch after opening the PR. That way you can copy the number and link.

[MSRV]: README.md#minimum-supported-rust-version
