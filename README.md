# Rust

- https://www.rust-lang.org/
- install

```shell
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source $HOME/.cargo/env

$ rustup --version

# update
$ rustup update

# compiler
$ rustc --version

# package manager
$ cargo --version
```

- vscode
  - install rust extension

# compile

```shell
$ rustc hello.rs
$ ./hello
```

# new project

```shell
$ cargo new project
# or
$ mkdir project
$ cargo init
# write code, compile and run
$ cd project
$ cargo run
# build
$ cargo build
# build production and optimization
$ cargo build --release
```
