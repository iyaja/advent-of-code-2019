# Advent of Code 2019

This repo contains my solutions to the 2019 Advent of Code challenges. The solutions are implemented in rust, with one file for each day (in the `src` folder). The solutions for each day can be verified and benchmarked using [`cargo aoc`](https://github.com/gobanos/cargo-aoc), a command line tool that provides useful Advent of Code helper functionality cargo.

## Setup

Since the solutions are implemented in Rust, you're going to need a few Rust tools like cargo and rustc to actually compile and run the code. Follow the [official Rust installation instructions](https://www.rust-lang.org/tools/install) to setup Rust if you haven't already.

Before evaluting the solutions, you also need to have `cargo aoc` installed. Follow the instructions from the official README, or simply run the following command with cargo installed:

```
cargo install cargo-aoc
```

Of course, 

## Running the solutions

Running the `cargo aoc` command at the base folder will compile and run all the solutions. The output will look similar to this:

```
AOC 2019
Day 1 - Part 1 : 3405637
        generator: 51.824µs,
        runner: 20.451µs

Day 1 - Part 2 : 5105597
        generator: 21.686µs,
        runner: 18.274µs
```

To learn more about the generator and runner timings, refer to the official `cargo-aoc` [docs](https://github.com/gobanos/cargo-aoc#setting-up-the-project). The total time will be the sum of the generator and runner times.
