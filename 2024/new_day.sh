#!/bin/bash

dir="d${1}"

if [ -d $dir ]; then
    echo "Directory already exists!";
    return 1;
fi

cargo new $dir
cd $dir

touch src/part1.rs src/part2.rs test_input.txt input.txt

echo "#[allow(dead_code)]
mod part1;
#[allow(dead_code)]
mod part2;

fn main() {
    part1::solve();
    // part2::solve();
}" > src/main.rs

echo "use aoc_helper::read_file;

pub fn solve() {
    let input = read_file(\"test_input.txt\");
    // let input = read_file(\"input.txt\");
}" > src/part1.rs

echo "aoc_helper = { git = \"https://github.com/GrownPlanet/aoc_helper.git\", tag = \"v0.1.5\" }" >> Cargo.toml

cargo b
