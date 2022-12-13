# Advent of Code 2022 [![advent_of_code_2022](https://github.com/ExcaliburZero/advent_of_code_2022/actions/workflows/main.yml/badge.svg)](https://github.com/ExcaliburZero/advent_of_code_2022/actions/workflows/main.yml)
These are my solutions to the problems for Advent of Code 2022.

https://adventofcode.com/2022

| Sun | Mon | Tue | Wed | Thr | Fri | Sat |
|----|----|----|----|----|----|----|
| | | | | [1](#day-1) | [2](#day-2) | [3](#day-3) |
| [4](#day-4) | [5](#day-5) | [6](#day-6) | [7](#day-7) | [8](#day-8) | [9](#day-9) | [10](#day-10) |
| [11](#day-11) | [12](#day-12) | [13](#day-13) | 14 | 15 | 16 | 17 |
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

## [Day 8](src/eight.rs)
Another one where the solutions are pretty simple, but the implementations are complex.

For part 1 we parse in the input into a 2d grid of integers and then check each point in the grid to see if it is visible. To do that we first check if the point is on an edge, if so it is visible. If it is not on an edge, we try "approaching" it from each direction (up, down, left, and right) where we start at each edge of the grid (level with the point) and keep approaching the point by moving in the direction until we either reach a hight at or above that of the destination point (not visible) or we reach the destination point (visible). We then return the number of visible points we found. 

For part 2 we parse in the grid in the same way and calculate the "senic score" of each point in the grid. To do that we start moving in each direction until we reach a tree that is at or above the height of our starting tree, tracking the number of visible trees we passed or reached and then multiplying those counts for each of the four directions. We then return the largest scenic score we found.

## [Day 9](src/nine.rs)
A state updating / position calculating problem. Definitely more fun than the previous few days.

For part 1 we parse in the input moves and generate our default "board state" with head and tail at (0, 0) and a record that the tail has visited (0, 0). For each move we update the state of the board one movement unit at a time. To update the state we move the head by that movement unit in the given direction and then update the position of the tail based on the rules in the problem description, and we also record the new tail position in the visited set. Once we have done that for each move we return the length of the tail visited positions set we built up along the way.

For part 2 we do the same, but instead of just having a head and tail we have a head, 8 inner segments, and a tail. We change the state "update" function to move the head, then aply the effects of the head movement to each of the inner segments in turn (using same tail update rules from part 1), then apply the last segment's movement effect to the tail and update the tail visited set with it's new position. Once we have done that for each move we return the length of the tail visited positions set we built up along the way.

## [Day 10](src/ten.rs)
Ooh, a simple CPU simulation! One of my favorites.

For part 1 we parse in the input into a list of instructions and create an X register and initialize it to `1`. We then need to execute each instruction cycle by cycle until we exhaust the instruction set. To do this we first create a Vec of each instruction along with the number of remaining cycles it has to execute, then for each cycle we grab the first remaining instruction, decrement its remaining cycles, and if it then has 0 remaining cycles we execute it on the register. To execute an instruction, if it is a `noop` we do nothing and if it is an `addx` we add the given value to the X register. To get the result we take the sum of the signal strength (cycle number * X value) for the cycles: 20, 60, 100, 140, 180, 220 (1 indexed), making sure to calculate the signal strength before executing the instruction (if one executes that cycle).

For part 2 we do the same, but instead of calculating signal stregnths we instead calculate pixel on and off values at each cycle. For each cycle we check if the value of X is within 1 of the column of the pixel for the current cycle `(cycle - 1) % 40` (again before executing any instruction for that cycle). We then build up a string out of those pixel values, making sure to add newlines after each 40th cycle.

## [Day 11](src/eleven.rs)
A problem with a mildly-fiddly implementation, and the part 2 was a bit tricky for me to figure out.

For part 1 we parse in the input (monkey -> items mapping, monkey item "update" rules, monkey item "transfer" rules) then we need to simulate each round. For each round we keep track of the monkey -> items mapping for the current round and for the next round. During a round we start with monkey `0` and continue until the last monkey, for each monkey we look at its items, for each item we apply the monkey's "update" rule and then divide the result by 3, then we find the destination by applying the monkey's "transfer" rule to that result, if the destination monkey is higher numbered than the current monkey we give the destination monkey the item this round, else we give the item next round. While doing the rounds we keep track in a count table of how many times each monkey inspected an item, then after all rounds have ended we return the product of the two largest monkey inspection counts.

For part 2 we do pretty much the same thing. Just instead of dividing by 3 after applying the "update" rule we instead take its remainder against our resolution (product of all monkey divisor values in their "transfer" rule) in order to keep the item values from getting too big while retaining the mathematical properties that we want. We also need to make sure we are using 64 bit (signed) integers for the item values and the monkey inspection counts.

## [Day 12](src/twelve.rs)
A pretty classic breadth-first search problem.

For part 1 we parse in the input into a 2D grid, converting `S` to `0` and recording it as the start position, `E` to `25` and recording it as the end position, and any other characters (`a` - `z`) as (`0` - `25`) respectively (convert to ascii and subtract ascii for `a`). Then we do a breadth-first search (BFS) from the start position to the end position and return the length of the found path (which is the shortest since the graph is unwighted). During the BFS we compute neighbors of a given position by taking the positions one step in the 4 cardinal directions and considering them neighbors if they are within the grid and steppable (source height + 1 >= destination height).

For part 2 we do the same, but rather than do just one BFS from the start to the end, we find each `0` height position in the grid, do a BSF search from each of those to the end position, and return the shortest possible path length found. We also have to make sure to handle `0` height positions that have no valid path to the end position (ignore them).

## [Day 13](src/thirteen.rs)
A combination stack-based parsing and recursive arbitrarily-structured list comparison problem.

To parse in the input for both parts we parse in each set of 2 input lines as their own lists. For each list we parse its input line using a stack to keep track of where we are in the list and go token by token (`"["`, `"]"`, `","`, integer), appending integers to the current list and on `[` and `]` going one list deeper or shallower respectively (we can ignore `","` tokens). Then when we have finished parsing the line, we record the remaining list in the stack as the parsed list.

For part 1 we need to check each of the pairs of lists to see if the first one is Less, Equal, or Greater than the second one. To do that we implement the recursive comparison rules described in the problem. Then we return the sum of the indicies of the pairs where the first list was Less or Equal to the second (one indexed).

For part 2 we create a Vec with all of the lists plus the 2 divider packets. Then we use the comparison function we implemented for part 1 to sort the Vec (ascending). Then we just do a search through the Vec to find the indices of the two divider packets (one indexed) and return the product of those 2 indices.