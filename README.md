# Advent of Code 2023 Solutions in Rust

This repository contains my personal solutions for the [Advent of Code 2023](https://adventofcode.com/2023) challenges, written in Rust.
Advent of Code is an annual set of Christmas-themed programming puzzles that cover a variety of skill sets and challenge types.

## Structure of the Repository

Each day's challenge is stored in a separate directory named `day_xx` where `xx` is the day number. 
Inside each directory, you will find:

- Input data automatically fetched using a bash script.
- My solution code in Rust.

## Setup Script

I use a custom bash script to set up each day's challenge environment. This script:

- Creates a new Rust project for the challenge.
- Fetches the challenge input to `input.txt` from the Advent of Code website.

**Note:** To use this script, a `sessionCookie` file containing your Advent of Code session cookie must be present in the root directory.

## Running the Code

To run a solution:

1. Ensure you have Rust installed.
2. Navigate to the challenge directory, e.g., `cd day_01`.
3. Run the solution using `cargo run`.

To set up a new day's challenge:

1. Run the setup script with the day number and part, e.g., `./setup.sh 1`.
2. Ensure you have the `sessionCookie` file in the root directory with your AoC session cookie.

