# Advent of Code 2022

My journey of using advent of code to learn Rust.
Maybe I'll do another one with AoC23

## Notes

### Day 01
* You can cast types using parse.
* You can solve max problems sorting a vector of values.
  - [ ] Check efficiency later when compared to storing the current max and comparing to a global max.
---

### Day 06
#### Benchmarking results

| Solution       | Runtime PT1 | Runtime PT2 |
|----------------|-------------|-------------|
| Chars HashSet  | 1.844 ms    | 3.786 ms    |
| Bytes HashSet  | 1.884 ms    | 7.702 ms    |
| Sliding Window | 141.096 μs  | 110.386 μs  |

#### Discussion

Surprisingly, my initial solution wasn't the worst performing one. I expected to be slower then the bites approach but I was proved wrong.
The sliding window outperformed the other solutions by far, since it's O(1)
while the others are O(n * m) where m = message size.

Source: 
* [Bytes Solution](https://github.com/dmshvetsov/adventofcode/blob/master/2022/06/2.rs)
* [Sliding window Solution](https://www.reddit.com/r/adventofcode/comments/zdw0u6/comment/iz42lhq/?utm_source=reddit&utm_medium=web2x&context=3)

