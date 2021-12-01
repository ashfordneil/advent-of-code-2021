# Advent of Code, 2021

This is my first attempt in a few years, but we'll see how it goes.

## Running a Solver

Everything is written in rust, so make sure you have that installed and up-to-date. By default, it expects you to
populate the `./data` directory with files like `day-one.txt` and so on, but you can just add a file-name to the command
below and it'll read the input from there instead.

```shell
$ cargo run --bin dayX
$ # alternatively, you can add an argument
$ cargo run --bin dayX path/to/input/file.txt
```

## Running the Tests

If advent of code is going to give me an example input and answer, I'm going to plug it into a testing framework to make
sure my code works.

```shell
$ cargo test
```