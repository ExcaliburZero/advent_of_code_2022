# Advent of Code 2022 [![advent_of_code_2022](https://github.com/ExcaliburZero/advent_of_code_2022/actions/workflows/main.yml/badge.svg)](https://github.com/ExcaliburZero/advent_of_code_2022/actions/workflows/main.yml)
These are my solutions to the problems for Advent of Code 2022.

https://adventofcode.com/2022

## Day 1
Pretty simple problem, got a little bit triped up at first by the input parsing since I thought they would have put the numbers for a elf on the same line but not bad.

For part 1 we just need to sum up each of the elves' food individually and then find the largest resulting number.

For part 2 we do pretty much the same thing, but we need to sum the three largest totals. I ended up just turning the totals into a `Vec`, sorting it (ascending), grabbing just the last three, and summing thos values.