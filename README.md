# Competitive Programming

This repository features solutions to Google's competitions and Codeforces using Rust.

## Content

Solutions to Kick Start Problems, Codeforces and CodeJam are all mixed up in the `src/` folder.

## Quick start

In this competitions speed is everything, that's why there's a file, `src/example.src`, which can be used to start solving a problem.

It contains number parsing, and vector parsing for `i32`, `usize`, `u32`, etc. All generics that implement `FromStr`.

It is also possible to simply read the next line, as a `String`, using `nxt`.

## `spr`

`spr` is a Rust CLI to simplify and speed up problem solving.

It has two modes, `create` and `run`. The default mode is to `run`.

To create a new problem:

```
cargo run <problem-name> -m create
```

This copies the contents of `src/example.rs` to `src/<problem-name>.rs` and also creates a test file `tests/<problem-name>.in`.

To run a problem:

```
cargo run <problem-name>
```

This runs `rustc src/<problem-name>.rs --out-dir rls` and then pipes `tests/<problem-name>.in` to `./rls/<problem-name>`. Assuming things go well. If an error should ocurr at any stage, the error message floats up to the stdout.

## Legacy

### CodeJam 2019

Solved enough to pass to next round. My two solutions worked flawlessly on the first attempt, and also passed the hidden test cases.

After the qualification, as practice, I solved problem 3, `Cryptopangrams`, inside the `cypher.js` file. Problem 4 is also solved in `datbae.js`, very tough problem!

### Transpiling

> CodeJam 2019 has enabled nodejs v11.13.0!

To use transpiling checkout the v1 tag.


