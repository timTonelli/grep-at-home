Can we stop and get some `grep`?

Mom: we have `grep` at home

[`grep` at home](https://github.com/timTonelli/grep-at-home)

This project is building on the [Rust CLI book](https://rust-cli.github.io/book/) exercise.

# Installation

Prerequisites:
* Rust
* Cargo
* Git

```shell copy
git clone https://github.com/timTonelli/grep-at-home.git
cd grep-at-home
cargo build --release
cargo install --path .
```

# Usage

`test.txt` can be used to test the tool out:

```shell copy
gah foo test.txt
```

or:

```shell copy
cat test.txt | gah foo
```

Both of the above example should yield `foo: 10`
