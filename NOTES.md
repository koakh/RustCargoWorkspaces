# NOTES

## Links

- [Cargo Workspaces - The Rust Programming Language](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)

## TLDR

### Creating a Workspace

```shell
$ mkdir add
$ cd add
```

`Cargo.toml`

```toml
[workspace]

members = [
  "adder",
]
```

```shell
# create the adder binary crate
$ cargo new adder
```

### Creating the Second Package in the Workspace

create another member package in the workspace and call it `add-one`

```toml
[workspace]

members = [
  "adder",
  "add-one",
]
```

```shell
# generate a new library 
$ cargo new add-one --lib
```

`add-one/src/lib.rs`

```rust
pub fn add_one(x: i32) -> i32 {
  x + 1
}
```

Now that we have another package in the workspace, we can have the `adder` package with our binary depend on the `add-one` package, that has our library. First, we’ll need to add a path dependency on `add-one` to `adder/Cargo.toml`.

`adder/Cargo.toml`

```toml
[dependencies]
add-one = { path = "../add-one" }
```

Next, let’s use the `add_one` function from the `add-one` crate in the `adder` crate.

`adder/src/main.rs`

```rust
use add_one;

fn main() {
  let num = 10;
  println!(
    "Hello, world! {} plus one is {}!",
    num,
    add_one::add_one(num)
  );
}
```

Let’s build the workspace by running `cargo build` in the top-level add directory!

```shell
$ cargo build
```

To run the binary crate from the `add` directory, we can specify which package in the workspace we want to run by using the `-p` argument and the package name with cargo run:

```shell
$ cargo run -p adder
```

### Depending on an External Package in a Workspace

`add/add-one/Cargo.toml`

```toml
[dependencies]
rand = "0.8.3"
```

add `use rand;` to

`add/add-one/src/lib.rs`

We can now add `use rand;` to the `add-one/src/lib.rs` file, and building the whole workspace by running `cargo build` in the `add` directory will bring in and compile the `rand` crate. We will get one warning because **we aren’t referring to the rand we brought into scope**:

```shell
$ cargo build
   Compiling add-one v0.1.0 (/mnt/storage/Development/Rust/@TheBook/RustCargoWorkspaces/add/add-one)
warning: unused import: `rand`
```

The top-level `Cargo.lock` now contains information about the dependency of `add-one` on `rand`. However, even though `rand` is used somewhere in the workspace, **we can’t use it in other crates in the workspace** unless we add `rand` to their `Cargo.toml` files as well. For example,  if we add `use rand;` to the `adder/src/main.rs` file for the `adder` package, we’ll get an error:

```shell
$ cargo build
  --snip--
   Compiling adder v0.1.0 (file:///projects/add/adder)
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```

To fix this, edit the `Cargo.toml` file for the `adder` package and indicate that `rand` is a dependency for it as well. Building the `adder` package will add `rand` to the list of dependencies for `adder` in `Cargo.lock`, but no additional copies of `rand` will be downloaded. Cargo has ensured that every crate in every package in the workspace using the `rand` package will be using the same version. Using the same version of `rand` across the workspace saves space because we won’t have multiple copies and ensures that the crates in the workspace will be compatible with each other.

## Adding a Test to a Workspace

For another enhancement, let’s add a test of the `add_one::add_one` function within the `add_one` crate:

`add-one/src/lib.rs`

```rust
pub fn add_one(x: i32) -> i32 {
  x + 1
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(3, add_one(2));
  }
}
```

Now run cargo test in the top-level add directory:

```shell
$ cargo test
```

The first section of the output shows that the `it_works` test in the `add-one` crate passed. The next section shows that zero tests were found in the `adder` crate, and then the last section shows zero documentation tests were found in the add-one crate. Running `cargo test` in a workspace structured like this one will run the tests for all the crates in the workspace.

We can also run tests for one particular crate in a workspace from the top-level directory by using the `-p` flag and specifying the name of the crate we want to test:

```shell
$ cargo test -p add-one
```

This output shows cargo test only ran the tests for the `add-one` crate and didn’t run the `adder` crate tests.

## Add an add-two crate to this workspace

```shell
$ cargo new add-two --lib
```

add `"add-two"`

```toml
[workspace]

members = [
  "adder",
  "add-one",
  "add-two",
]
```

`add/add-two/src/lib.rs`

```rust
pub fn add_two(x: i32) -> i32 {
  x + 2
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(4, add_two(2));
  }
}
```

`add/adder/src/main.rs`

add 

```rust
use add_two;

fn main() {
  println!(
    "Hello, world! {} plus two is {}!",
    num,
    add_two::add_two(num)
  );
}
```

```shell
$ cargo run -p adder
Hello, world! 10 plus one is 11!
Hello, world! 10 plus two is 12!
```

done