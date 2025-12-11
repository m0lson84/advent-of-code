<img src="./.assets/christmas_ferris.png" width="164">

# advent-of-code

[![CI](https://github.com/m0lson84/advent-of-code/actions/workflows/ci.yml/badge.svg)](https://github.com/m0lson84/advent-of-code/actions/workflows/ci.yml)

<!--toc:start-->

- [advent-of-code](#advent-of-code)
  - [Getting Started](#getting-started)
    - [Install](#install)
  - [Development](#development)
    - [Scaffolding](#scaffolding)
    - [Input Downloads](#input-downloads)
    - [Project Structure](#project-structure)
    - [Running Solutions](#running-solutions)
    - [Formatting & Linting](#formatting-linting)
  - [Testing](#testing)
  - [Benchmarking](#benchmarking)
  - [Results](#results)
  - [Benchmarks](#benchmarks)
  <!--toc:end-->

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

## Getting Started

This section provides some guides on how to stand up your development environment.

### Install

This project is written in Rust and requires that it be installed prior to development. The recommended method is to use
[rustup](https://rustup.rs/), the official Rust toolchain installer. Alternatively, you can use a version manager like
[mise-en-place](https://mise.jdx.dev/), or a package manager like [brew](https://brew.sh/) or
[apt](https://ubuntu.com/server/docs/package-management).

## Development

### Scaffolding

To scaffold a new day's solution, run the following command:

```bash
# example: `cargo scaffold 1`
cargo scaffold <day>

# output:
# Created module file "src/bin/01.rs"
# Created empty input file "data/inputs/01.txt"
# Created empty example file "data/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
```

This creates a solution file from the template along with empty data files for inputs and examples.

> [!TIP]
> If a day has multiple example inputs, you can use the `read_file_part()` helper in your tests instead of
> `read_file()`. If this e.g. applies to day 1, you can create a second example file `01-2.txt` and invoke the helper
> like `let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));`. This supports an
> arbitrary number of example files.

### Input Downloads

> [!IMPORTANT]
> This requires installing the [`aoc-cli`](https://github.com/scarvalhojr/aoc-cli/) crate via
> `cargo install aoc-cli --version 0.12.0`. You will also need to create the file
> `<home_directory>/.adventofcode.session` and paste your session cookie into it. To retrieve the session cookie, press
> F12 anywhere on the Advent of Code website to open your browser developer tools. Look in _Cookies_ under the
> _Application_ or _Storage_ tab, and copy out the `session` cookie value.

You can automatically download puzzle input and description by either appending the `--download` flag to `scaffold`
(e.g. `cargo scaffold 4 --download`) or with the separate `download` command:

```bash
# example: `cargo download 1`
cargo download <day>

# output:
# [INFO  aoc] 🎄 aoc-cli - Advent of Code command-line tool
# [INFO  aoc_client] 🎅 Saved puzzle to 'data/puzzles/01.md'
# [INFO  aoc_client] 🎅 Saved input to 'data/inputs/01.txt'
# ---
# 🎄 Successfully wrote input to "data/inputs/01.txt".
# 🎄 Successfully wrote puzzle to "data/puzzles/01.md".
```

### Project Structure

Individual solutions live in the `./src/bin/` directory as separate binaries. _Inputs_ and _examples_ live in the
`./data` directory.

Every solution has _tests_ referencing its _example_ file in `./data/examples`. Use these tests to develop and debug
your solutions against the example input. In VS Code, `rust-analyzer` will display buttons for running / debugging
these unit tests above the unit test blocks.

### Running Solutions

To run a solution for a specific day:

```bash
# example: `cargo solve 01`
cargo solve <day>

# output:
#     Finished dev [unoptimized + debuginfo] target(s) in 0.13s
#     Running `target/debug/01`
# Part 1: 42 (166.0ns)
# Part 2: 42 (41.0ns)
```

The `solve` command runs your solution against real puzzle inputs. To run an optimized build, append the `--release`
flag as with any other rust program.

To run all solutions sequentially:

```bash
cargo all

# output:
#     Running `target/release/advent_of_code`
# ----------
# | Day 01 |
# ----------
# Part 1: 42 (19.0ns)
# Part 2: 42 (19.0ns)
# <...other days...>
# Total: 0.20ms
```

### Formatting & Linting

To format the code:

```bash
cargo fmt
```

To lint the code:

```bash
cargo clippy
```

## Testing

To run tests for the application, use the following command:

```bash
cargo test
```

To run tests for a specific day, append `--bin <day>`, e.g. `cargo test --bin 01`. You can further scope it down to a
specific part, e.g. `cargo test --bin 01 part_one`.

## Benchmarking

To benchmark your solutions:

```bash
# example: `cargo time 8 --store`
cargo time <day> [--all] [--store]

# output:
# Day 08
# ------
# Part 1: 1 (39.0ns @ 10000 samples)
# Part 2: 2 (39.0ns @ 10000 samples)
#
# Total (Run): 0.00ms
#
# Stored updated benchmarks.
```

The `cargo time` command allows you to benchmark your code and store timings in the readme. When benching, the runner
will run your code between `10` and `10,000` times, depending on the execution time of the first run, and print the
average execution time.

`cargo time` has three modes of execution:

1. `cargo time` without arguments incrementally benches solutions that have not been stored in the readme yet and
   skips the rest.
2. `cargo time <day>` benches a single solution.
3. `cargo time --all` benches all solutions.

By default, `cargo time` does not write to the readme. In order to do so, append the `--store` flag:
`cargo time --store`.

> [!NOTE]
> Please note that these are not _scientific_ benchmarks, understand them as a fun approximation. 😉 Timings, especially
> in the microseconds range, might change a bit between invocations.

## Results

<!--- advent_readme_stars table --->

## 2025 Results

|                     Day                      | Part 1 | Part 2 |
| :------------------------------------------: | :----: | :----: |
| [Day 1](https://adventofcode.com/2025/day/1) |   ⭐   |   ⭐   |
| [Day 2](https://adventofcode.com/2025/day/2) |   ⭐   |   ⭐   |
| [Day 3](https://adventofcode.com/2025/day/3) |   ⭐   |   ⭐   |
| [Day 4](https://adventofcode.com/2025/day/4) |   ⭐   |   ⭐   |
| [Day 5](https://adventofcode.com/2025/day/5) |   ⭐   |   ⭐   |
| [Day 6](https://adventofcode.com/2025/day/6) |   ⭐   |   ⭐   |
| [Day 7](https://adventofcode.com/2025/day/7) |   ⭐   |   ⭐   |
| [Day 8](https://adventofcode.com/2025/day/8) |   ⭐   |   ⭐   |

<!--- advent_readme_stars table --->

<!--- benchmarking table --->

## Benchmarks

|           Day            |  Part 1   |  Part 2   |
| :----------------------: | :-------: | :-------: |
| [Day 1](./src/bin/01.rs) | `54.8µs`  | `38.0µs`  |
| [Day 2](./src/bin/02.rs) | `27.5ms`  | `117.3ms` |
| [Day 3](./src/bin/03.rs) | `34.8µs`  | `58.8µs`  |
| [Day 4](./src/bin/04.rs) | `123.5µs` |  `4.0ms`  |
| [Day 5](./src/bin/05.rs) | `90.8µs`  | `28.2µs`  |
| [Day 6](./src/bin/06.rs) | `146.8µs` | `69.3µs`  |
| [Day 7](./src/bin/07.rs) | `155.3µs` | `195.2µs` |
| [Day 8](./src/bin/08.rs) | `27.3ms`  | `120.9ms` |

**Total: 298.00ms**

<!--- benchmarking table --->
