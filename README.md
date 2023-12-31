# Advent of Code 2023

### 01

Use [`str::rfind`](https://doc.rust-lang.org/std/primitive.str.html#method.rfind) and [`Iterator::max_by_key`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by_key) to find rightmost matching pattern. 


### 03

Any `Number` is on a single row `y` and occupies a range `x0..=x1`. Find its surrouding neighbors with [`Itertools::cartesian_product`](https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.cartesian_product):
    
    ######
    #4321#
    ######


### 04

Given that Card<sub>i</sub> has `n` copies, and its count is `c`, each of the following Card<sub>i+1 </sub>..=Card<sub>i+c </sub> has its number of copies increased by `n`.


### 05

For each seed range `s1..s2`, filter it thru a range `r1..r2` to a potential `s1..r1..r2..s2`. Then split it into up to 3 groups `s1..r1`,`r1..r2`, and `r2..s2`. After each seed-split-thru-range step is done, grab the remaining non-split seed range and split it thru the next range in the same range group.

After split with the whole range group, the seed range is now in smaller ranges. Check if any of them fits any range, and update seed range with corresponding target range. 


### 06

a*x<sup>2</sup> + bx + c = 0


### 08

Each `xxA` corresponds to a unique `xxZ`, and the loop size is equal to length of `xxA..xxZ`. Find each loop size, and compute their Least Common Multiple.


### 10

Find the loop with BFS. For P2 shoelace and flood filling both work, here we use ray casting. Imagine casting a ray from the left and count the number of boundaries it crosses. If it is odd it lands inside the loop, otherwise ouside. So both `|` and `|||` counts as inside, but `┌─┐` would mean it's out, and `└─┐` effectively functions as `|`. Based on such observations, we count the times that a node pointing north/up appears in our path (but never both north AND south). Odd means inside, even means out. 


### 12

The HARD one, dynamic programming. 

For a target string with a mix of `'?', '#', '.'`, the naive brute-force solution is to try each `'?'` as `'.'` or `'#'` and recurse the rest of the string. Once no `?` exists, check whether current config matches the `pattern`. This would take eons to solve Part 2. 

The DP solution. For any string `value` and `pattern`, use both together as a key to cache recursion result. When in recursion, check that:
    
- End condition #1
    
    When `value.is_empty()`, if `pattern` is also empty, this means both are depleted and it is a match. Otherwise it is not.

- End condition #2
    
    `value.contains('#')` but `pattern` is empty. Match fails.

- End condition #3
    
    `value` is shorter than `pattern[0]`, i.e not enough potential `'#'`s. Match fails.

- End condition #4
    
    Fall thru from #3. `value` is long enough, but in the first `pattern[0]` chars/bytes there is at least one `'.'`. Match fails.

- End condition #5
    
    Fall thru from #3 and #4. If pattern has only one element and `value.len == pattern[0]`, match succeeds. 

- End condition #6
    
    Fall thru from #3, #4, and #5. `value` is longer than `pattern[0]` and its first `pattern[0]` chars are `'#'`, but its `pattern[0]` char is still `'#'`. Match fails.

- Recursion #1
    
    `value.starts_with('.')`, recurse the rest of the string. 
    
- Recursion #2
    
    `value.starts_with('?')`, either replace first char with `'#'` and recurse the whole string, or skip it and do basically #1.

- Recursion #3
    
    End #4, #5, and #6 guarantee that `value` starts with a block of `'#'`s whose length equals `pattern[0]` exactly, i.e after those `'#'`s no `'#'` follows immediately. Chop those `'#'` off `value` and `pattern[0]` off `pattern`, recurse the rest of those. 


### 13

Part 2's example tricked me into thinking that the spot has to be next to the mirror line, which is not the case. It can be anywhere on the mirror.


### 14

Not a smart way to cycle-tick the rocks. To find the loop, imaging the number goes

    start...loop_start1..loop_end1,loop_start2..loop_end2....end

The first repeating value at `loop_start2` shows that `loop_size = loop_start2 - loop_start1`. Now to find the last `loop_end`, chop off `start..` from the number range:

    loop_start1..loop_start2....loop_end..end

So `end-loop_end == (total_count - loopstart2) % loop_size`. Solve loop_end, fast forward to it, then loop to the end.


### 16

Another BFS. There must be a better way to do part 2. In theory the split nodes `'|'` and `'-'` are hot spots but couldn't find a way to work it in. Used [rayon](https://docs.rs/rayon/1.8.0/rayon/index.html) to speed up brute force. 


### 17

Dijkstra path finding, gladly taken from [Rust doc on BinaryHeap](https://doc.rust-lang.org/std/collections/binary_heap/#examples). Had to keep track of position, direction, and streak count as visited states. Different starting directions result in different starting states too. 

The difficult part lies in counting streak number. For maximum it's simple. For minimum, it checks the previous streak when changing direction: if it is too short, current state (direction change) is invalid and should be discarded. 


### 18

[Shoelace formula](https://www.themathdoctors.org/polygon-coordinates-and-areas/) and Pick's theorem. Flood fill works for part 1, for part 2 it blows up.


### 19

Regex is hard, and apparently different languages' support for regex varies😥 Part 2 is very similar to [day 5](#05): divide a big range into smaller ranges until meeting the final nodes. That is naturally recursive. 


### 20

Bowed to borrow checker and marked every string behind `&'static`. It's still better than allocating `String`s I guess. Part 1 is more of a reading comprehension problem, and I readied a loop detection, which might be overkill. Part 2 is very dependent on input shape, apparently carefully crafted. Find the node feeding into the last node, then dig one step back to find the input nodes. For each input node, find the loop size that satisfies and calculate their LCM. There's also an [SVG file](./d20/graph.svg) attached, generated with [Graphviz](https://graphviz.org/). 


### 21

Was misreading part 1. It asks the number of tiles reachable on the EXACT 64th step, not the total amount of tiles stepped on. Part 2 comes again with a very specific input and I still don't fully understand it. Shamelessly copied from [this post](https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21) and called it a day.


### 22

Read. Read carefully. Read all highlighted parts. Read not only the puzzle, but the input too. 


### 23

Part 2 is a "meta" BFS -- condense the graph first, and then traverse every path to find the longest. The queue keeps track of several states together: current node, current distance from start, and visited nodes on current path. 


### 24

Ah math and line intersections and slopes. Part 2 owes many thanks to [u/TheZigerionScammer's solution](https://topaz.github.io/paste/#XQAAAQAUDAAAAAAAAAA0m0pnuFI8c914retSmoIG4DZEdJ50slbD81JvM5mQSTreyJmJdG5ErENvWrbR2IGVD6L23kMHykcRMgYleThe4um56yMUrQ/uHrF3HuwBAoalVRDpkkpZviPXlbzmoSJoN4HPLXXSEz4to1kWUxZqDAP0KgxHB8lNPronrj59GR1o5RqFHlyAaZKCkCt0CT05d5nlzaQEh7vz9YXrWsW2L7GNJy0xlJasMTNGvbHeDPXyJItiHXa3MseDaV2MWdvPtI54e9x9dNc4x0wcP85QR7YrlVSs0zCm5rXLsb1nwhxW3xodqj8NIsH0KEVTE+RsDEuy9p9GD7XMP3k/ijz+cL4XqNUA16WFns+o63OLK8vjoiOK5hNuNhurOMMPFIZW6J4Gcf1a64jhwzu9ISgbCXSSR+Bds+Enp5Nwbt7ZwZ1dQ+Ht3zQ4fZeC6auWvC1ES+fsaFDO3vNSXhoUvOqqnk7jkpTDnwCCI3BDrpwD4ixNe9OOP3MMecfv0uWuZYp7IIgsVCQVXIFVmmhvdhQsZ0FmfBcbvK04YE8RMztc7U2dJ4gWw/yF69/CppBSQCPH4Kn0ZCtn0uYJiJXq9BbA7QiokCY4P+rK9k1S0QqL2nlmI1BqIZkboC3A/kV12oqDIfxCn3sylSN/NDGoXUhFaF+fwn7Q4tfyE9xnARW+3AxntYM6cMwc8ZcyyOlBnrM4iJgPsXteSvwdXl5b8YwEpUc/h+Y5JQp1PnFALM6GLx/q85mWShC+xF6KYfcJ4oWboeIVN9TYKhLU8m+MPFnqitqskmfRvaPb8LfK3OSdRFDZUg5N+wrGfxcdg8EtttL+/94x+9FAVz31BpkHQtwS5aMlUr1TpLphUbzn862x9UwmDlR3vhWBr/OeZ2FlQO3F01yGuH5MRytgVH7GHGDipyh7lXLjHA8L5RjuDDUZa7/gHUYHx0iW3dz7dC2bsSiWBpgAMP5YVdQhJMbVYyhP68Nw1H7hmHqwXB4u8k4QuXHeetPd3Z9lSUG4KSpjwy5ePYMdLaLI2KM1GInxXo3MC3rtKFvEr0NyV7ifJ8YBsmu9h26z8bv8qRN8SFBQ/IPrIcdrQI4AHXDg1hDr2641UlRFVGDTDebMhcKlR2nUEdko6UyNvb6FrmiwQTIGZT7E2Gb+X02v+DoWEaxLcG1Imdt6j/Tl9+PmcCa0P3V7/ucxC9GOAerwTAfcDEAhaUYOco9a0nxdRLDc9qbn8Fc3rvjsvTIoTwgykwqraj6xMWBdsCYs1/+CY+8A). The idea is:

- If two line has the same vel on an axis, e.g `vx1 == vx2`, the crossing line must cross them both. Its velx must satisfy `(pos_x2 - pos_x1) % (velx - vx1) == 0`
- Repeat that for all parallel pairs to narrow down `velx`, to a unique value. Find `vely` and `velz` too. 
- Now use `y = mx + b` to find `pos_x`.

Tried to use `z3` too, but it seems broken on Windows, ironically as an MS project. 
