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

