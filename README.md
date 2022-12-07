# Advent of Code 2022 [![advent_of_code_2022](https://github.com/ExcaliburZero/advent_of_code_2022/actions/workflows/main.yml/badge.svg)](https://github.com/ExcaliburZero/advent_of_code_2022/actions/workflows/main.yml)
These are my solutions to the problems for Advent of Code 2022.

https://adventofcode.com/2022

| Sun | Mon | Tue | Wed | Thr | Fri | Sat |
|----|----|----|----|----|----|----|
| | | | | [1](#day-1) | [2](#day-2) | [3](#day-3) |
| [4](#day-4) | [5](#day-5) | [6](#day-6) | [7](#day-7) | 8 | 9 | 10 |
| 11 | 12 | 13 | 14 | 15 | 16 | 17 |
| 18 | 19 | 20 | 21 | 22 | 23 | 24 |
| 25 | | | | | | |

## [Day 1](src/one.rs)
Pretty simple problem, got a little bit triped up at first by the input parsing since I thought they would have put the numbers for a elf on the same line but not bad.

For part 1 we just need to sum up each of the elves' food individually and then find the largest resulting number.

For part 2 we do pretty much the same thing, but we need to sum the three largest totals. I ended up just turning the totals into a `Vec`, sorting it (ascending), grabbing just the last three, and summing thos values.

## [Day 2](src/two.rs)
This one was pretty easy. A little fiddly with getting the details right, but not bad.

For part 1 we just need to sum up the points we get from the results of the match and the points we get for our move choices. Finding the move choice values is just a lookup. Finding the results of the match can be done with a lookup on all the possibilities and then map the result to the corresponding numerical value.

For part 2 we need to sum the same type of values, but rather than knowing the move and calculating the result we instead know the result and need to calculate the move. Again it's just a lookup on all the possible combinations.

## [Day 3](src/three.rs)
Ah, a good old set problem.

For part 1 we just need to break each line in half, make a set of the characters on each side, and take the intersection of those two sets to find the character we are interested in. Then we just check the case of the letter (upper vs. lower) as covert it to ascii and shift it accordingly to calcuate its priority. We then do that for each line of input and sum the resulting priorities.

For part 2 we do something quite similar, but instead of splitting the lines each in half we take three lines at a time, turn them each into their own character set, intersect those three sets, and convert that intersection character to its priority value (same way as part 1). We just do that for each group of three lines and sum the resulting priorities.

## [Day 4](src/four.rs)
A pretty straightforward ranges problem.

For part 1 we just need to count the number of pairs of ranges that fully contain one or another. We loop over the pairs of ranges `(a, b)` and apply a containment check on both `(a, b)` and `(b, a)` where if either is true we count it towards our total. The containment check is a simple `a_s <= b_s && a_e >= b_e`.

For part 2 we do that same thing but count the number of overlapping pairs of ranges. We do the same loop over the pairs of ranges but apply the following overlap check: `a_s <= b_e && b_s <= a_e`.

## [Day 5](src/five.rs)
A pretty standard stacks problem.

For part 1 we just need to process each "move" by repeatedly poping a value off of the source crate and pushing it onto the end of the destination crate the specified number of times. Then we just concatenate together the characters at the end of each create to get the answer.

For part 2 we do the same but when processing each "move" instead of doing each count of the move separately we do them in one go, where we pop that many elements off of the source crate and push them onto a temporary stack then reverse the temporary stack and pop the elements off of the temporary stack and push them onto the destination crate.

## [Day 6](src/six.rs)
A nice and easy buffer/deque problem.

For part 1 we just have to iterate over the input character by character, accumulating a buffer of the characters that we see. Once we reach 4 elements in the buffer we check if they are all unique, if so return the current index in the input + 1, else we remove the first element in the buffer and try again with the next character in the input.

For part 2 we do the same thing but with 14 characters instead of 4.

## [Day 7](src/seven.rs)
Hmmm... I'm not sure if this was a large complexity spike or I just overcomplicated things...

For part 1 we parse the commands from the input and then "execute" the commands to build up a representation of the directory structure. Then we recursively iterate over the directory structure and sum up the sizes of all directories that are above 100,000 in size. We then return that sum.

For part 2 we parse and "execute" to build up the directory structure like in part 1. Then we recursively iterate over the directory structure but instead try to find the size of the smallest directory that is at least `root.size() - (70,000,000 - 30,000,000)` in size.