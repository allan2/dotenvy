# Contributing to _dotenvy_

**Thank you very much for considering to contribute to this project!**

We welcome any form of contribution:

- Asking for help
- Reporting a bug
- Adding a feature
- Fixing a problem
- Improving the code
- Sharing ideas
- Adding documentation
- Adding examples
- Fixing spelling/grammar errors
- _etc..._

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

Please feel free to open an [issue](https://github.com/allan2/dotenvy/issues),
or comment on existing issues regarding any topic. If you would like to attempt
to implement a change, you can ask for help or guidance on solving a problem, or
just let us know you are working on it.

A maintainer will mark the issue as assigned to you to avoid other people
unknowingly working on the same thing. If you notice an old assigned issue still
has no update, please still ask. We can reassign as necessary.

This project sometimes has longer form
[discussions](https://github.com/allan2/dotenvy/discussions). Feel free to
comment on existing threads or open your own.

## Pull Requests

Pull Requests are the way concrete changes are made to the code, documentation,
and dependencies in the dotenvy repository.

Even tiny pull requests (e.g., one character pull request fixing a typo in API
documentation) are greatly appreciated. Before making a large change, it is
usually a good idea to first open an issue describing the change to solicit
feedback and guidance. This will increase the likelihood of the PR getting
merged.

### Tests

If the change being proposed alters code (as opposed to only documentation for
example), it is either adding new functionality to a crate or it is fixing
existing, broken functionality. In both of these cases, the pull request should
include one or more tests to ensure that the crate does not regress in the
future.

### Discuss and update

You will probably get feedback or requests for changes to your Pull Request.
This is a big part of the submission process so don't be discouraged! Some
contributors may sign off on the Pull Request right away, others may have more
detailed comments or feedback. This is a necessary part of the process in order
to evaluate whether the changes are correct and necessary.

Any community member can review a PR and you might get conflicting feedback.
Keep an eye out for comments from code owners to provide guidance on conflicting
feedback.

#### Commit Squashing

In most cases, do not squash commits that you add to your Pull Request during
the review process. When the commits in your Pull Request land, they may be
squashed into one commit per logical change. Metadata will be added to the
commit message (including links to the Pull Request, links to relevant issues,
and the names of the reviewers). The commit history of your Pull Request,
however, will stay intact on the Pull Request page.

### CI

Our continuous integration workflow checks all pull requests to ensure:

- All tests pass on stable and [MSRV]
- No [clippy](https://github.com/rust-lang/rust-clippy) errors or warnings
- [Rustfmt](https://github.com/rust-lang/rustfmt) is adhered to
- [Rustdoc](https://doc.rust-lang.org/rustdoc/write-documentation/linking-to-items-by-name.html)
  links work.

Before submitting a PR, it may be worth running these checks yourself.

#### MSRV

Check the [readme][MSRV] for the current
minimum supported rust version. We are open to updating it.

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