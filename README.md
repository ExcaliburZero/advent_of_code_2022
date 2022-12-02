# Advent of Code 2022 [![advent_of_code_2022](https://github.com/ExcaliburZero/advent_of_code_2022/actions/workflows/main.yml/badge.svg)](https://github.com/ExcaliburZero/advent_of_code_2022/actions/workflows/main.yml)
These are my solutions to the problems for Advent of Code 2022.

https://adventofcode.com/2022

| Sun | Mon | Tue | Wed | Thr | Fri | Sat |
|----|----|----|----|----|----|----|
| | | | | [1](#day-1) | [2](#day-2) | 3 |
| 4 | 5 | 6 | 7 | 8 | 9 | 10 |
| 11 | 12 | 13 | 14 | 15 | 16 | 17 |
| 18 | 19 | 20 | 21 | 22 | 23 | 24 |
| 25 | | | | | | |

## Day 1
Pretty simple problem, got a little bit triped up at first by the input parsing since I thought they would have put the numbers for a elf on the same line but not bad.

For part 1 we just need to sum up each of the elves' food individually and then find the largest resulting number.

For part 2 we do pretty much the same thing, but we need to sum the three largest totals. I ended up just turning the totals into a `Vec`, sorting it (ascending), grabbing just the last three, and summing thos values.

## Day 2
This one was pretty easy. A little fiddly with getting the details right, but not bad.

For part 1 we just need to sum up the points we get from the results of the match and the points we get for our move choices. Finding the move choice values is just a lookup. Finding the results of the match can be done with a lookup on all the possibilities and then map the result to the corresponding numerical value.

For part 2 we need to sum the same type of values, but rather than knowing the move and calculating the result we instead know the result and need to calculate the move. Again it's just a lookup on all the possible combinations.