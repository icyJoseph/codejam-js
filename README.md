# Competitive Programming

This repository features solutions to Google's competitions and Codeforces using Rust.

## Content

Solutions to Kick Start Problems, Codeforces and CodeJam are all mixed up in the `src/` folder.

## Quick start

In this competitions speed is everything, that's why there's a file, `src/example.src`, which can be used to start solving a problem.

It contains number parsing, and vector parsing for `i32`, `usize`, `u32`, etc. All generics that implement `FromStr`.

It is also possible to simply read the next line, as a `String`, using `nxt`.

### Toolbox

#### `nxt`

Reads a line from `stdin` and returns it as a `String`.

```rust
fn main() -> Res<()> {
    let string = nxt();
    println!("{}", string);
}
```

#### `parse_num`

Builds upon `nxt`, to parse a line as a number into type `T: std::str::FromStr`.

```rust
fn main() -> Res<()> {
    let num = parse_num::<u32>();
    println!("{}", num);
}
```

#### `parse_vec`

Builds upon `nxt`, to parse a line as a vector of type `T: std::str::FromStr`.

```rust
fn main() -> Res<()> {
    let vector = parse_vec::<u32>();
    println!("{:?}", vector);
}
```

## `spr`

`spr` is a Rust CLI to simplify and speed up problem solving.

It has two modes, `create` and `run`. The default mode is to `run`.

### Compiling

From the root of the project:

```
cd spr
```

Build the binary:

```
cargo build --release
```

### Creating a new problem

From the root of the project:

```
./spr/target/release/spr <problem-name> -m create
```

This copies the contents of `src/example.rs` to `src/<problem-name>.rs` and also creates a test file `tests/<problem-name>.in`.

### Run a problem

From the root of the project:

```
./spr/target/release/spr <problem-name>
```

This runs `rustc src/<problem-name>.rs --out-dir rls` and then pipes `tests/<problem-name>.in` to `./rls/<problem-name>`.

- If an error should occur at any stage, the error message shows in the `stdout`.
- If all goes well, then your program output shows in the `stdin`.

## Legacy

### CodeJam 2019

Solved enough to pass to next round. My two solutions worked flawlessly on the first attempt, and also passed the hidden test cases.

After the qualification, as practice, I solved problem 3, `Cryptopangrams`, inside the `cypher.js` file. Problem 4 is also solved in `datbae.js`, very tough problem!

### Transpiling

> CodeJam 2019 has enabled nodejs v11.13.0!

To use JavaScript transpiling checkout the v1 tag.
