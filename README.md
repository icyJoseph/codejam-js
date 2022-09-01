# Competitive Programming

This repository features solutions to Google's competitions and Codeforces using Rust.

## Content

Solutions to Kick Start Problems, Codeforces and CodeJam are all mixed up in the `src/` folder.

## Quick start

In this competitions speed is everything, that's why there's a file, `template/src/main.src`, which can be used to start solving a problem.

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

## Creating a new problem

Enter to the `src` folder and run:

```
cargo generate --path ./template --name <problem_name> --force
```

## Solving a problem

Place your solution inside the `main` function. Create as many helpers as necessary.

## Run a problem

From the problem directory:

```
cargo build --release --target-dir .
./release/<problem_name> < <problem_name>.in
```

## Legacy

### CodeJam 2019

Solved enough to pass to next round. My two solutions worked flawlessly on the first attempt, and also passed the hidden test cases.

After the qualification, as practice, I solved problem 3, `Cryptopangrams`, inside the `cypher.js` file. Problem 4 is also solved in `datbae.js`, very tough problem!

### Transpiling

> CodeJam 2019 has enabled nodejs v11.13.0!

To use JavaScript transpiling checkout the v1 tag.
