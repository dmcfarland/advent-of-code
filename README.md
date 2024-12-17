# Advent of Code

Progress with https://adventofcode.com/

# Running samples as tests
```
cargo test --bin 2024-day-01 -- --nocapture
```

# Running against puzzle input
Expects puzzle input in file input/{year}-day-{day}.txt i.e. input/2024-day-01.txt
```
cargo run --bin 2024-day-01
```

## Progress
Works - ✅
Slow - 🐌
Guessed - 🎲
Broken - ⛔
Not started - ⭕
In Progress - 🚧

| Day | Part 1 | Part 2 | Notes |
|-----|:------:|:------:|-----|
| 01  |   ✅   |   ✅  |      |
| 02  |   ✅   |   ✅  |      |
| 03  |   ✅   |   ✅  |      |
| 04  |  🚧🎲  |   ✅  | Part 1 off by 4 |
| 05  |   ✅   |   ✅  |      |
| 06  |   ✅   |   ✅  | 5s   |
| 07  |   🚧   |   🚧  | Broken     |
| 08  |   ✅   |   ✅  |      |
| 09  |   🐌   |   ✅  | Part 1 17s     |
| 10  |   ✅   |   ✅  |      | >>>>>>>
| 11  |   🐌   |   ⭕  | Part 1 36s     |
| 12  |   ⭕   |   ⭕  |      |
| 13  |   🚧   |   ⭕  | Started      |
| 14  |   ✅   |   ✅  |      |
| 15  |   ✅   |   ✅  |      |
| 16  |   🐌   |   🐌🎲  | Part 1 - 8s, Part 2 - [6m58s & guessed pruning offset] |
| 17  |   ✅   |   🐌🎲  | Part 2 - [14s, Guessed at initial offset]    |
| 18  |      |     |      |
