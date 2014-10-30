rust-dotenv
====

**Achtung!** This is a v0.\* version! Expect bugs and issues all around.
Submitting pull requests and issues is highly encouraged!

Quoting [bkeepers/dotenv][dotenv]:

> Storing [configuration in the environment](http://www.12factor.net/config)
> is one of the tenets of a [twelve-factor app](http://www.12factor.net/).
> Anything that is likely to change between deployment environments–such as
> resource handles for databases or credentials for external services–should
> be extracted from the code into environment variables.

This library is meant to be used on development or testing environments in
which setting environment variables is not practical. It loads environment
variables from a `.env` file, if available, and mashes those with the actual
environment variables provided by the operative system.

Usage
----

The easiest and most common usage consists on calling `dotenv::dotenv` when the
application starts, which will load environment variables from a file named
`.env` located wherever your application binary is located; after that, you
can just call the environment-related method you need as provided by `std::os`.

If you need finer control about the name of the file or its location, you can
use the `from_filename` and `from_path` methods provided by the crate.

Examples
----

A `.env` file looks like this:

```sh
# a comment, will be ignored
REDIS_ADDRESS=localhost:6379
MEANING_OF_LIFE=42
```

You can optionally prefix each line with the word `export`, which will
conveniently allow you to source the whole file on your shell.

A sample project using Dotenv would look like this:

```rust
extern crate dotenv;

use dotenv::dotenv;
use std::os::env;

fn main() {
    dotenv().ok();

    for (key, value) in env().into_iter() {
        println!("key: {}, value: {}", key, value)
    }
}
```

Issues
----

* The way errors are implemented is not as nice as it could be. Now that
[FromError][fromerror] has landed, a better design for `DotenvError` and
`ParseError` is possible.

[dotenv]: https://github.com/bkeepers/dotenv
[fromerror]: https://github.com/aturon/rfcs/blob/error-chaining/active/0000-error-chaining.md
