# Brainfuck ZKVM

- [x] build brainfuck vm with compiler
- [ ] build brainfuck vm with stark
  - [x] Execution Trace
  - [ ] STARK Prover
    - [ ] Write Constraint
  - [ ] STARK Verifier

### Compiler

```bf
++>,<[>+.<-]
```

```sh
[43, 43, 62, 44, 60, 91, 13, 62, 43, 46, 60, 45, 93, 7]
```

### Execution Trace

Trace:

```sh
clk:0, ip: 0, ci: 43, ni: 43, mp: 0, mv: 0, mvi: 0
clk:1, ip: 1, ci: 43, ni: 62, mp: 0, mv: 1, mvi: 1
clk:2, ip: 2, ci: 62, ni: 44, mp: 0, mv: 2, mvi: 9223372034707292161
clk:3, ip: 3, ci: 44, ni: 60, mp: 1, mv: 0, mvi: 0
clk:4, ip: 4, ci: 60, ni: 91, mp: 1, mv: 97, mvi: 15023636922512908880
clk:5, ip: 5, ci: 91, ni: 13, mp: 0, mv: 2, mvi: 9223372034707292161
clk:6, ip: 7, ci: 62, ni: 43, mp: 0, mv: 2, mvi: 9223372034707292161
clk:7, ip: 8, ci: 43, ni: 46, mp: 1, mv: 97, mvi: 15023636922512908880
clk:8, ip: 9, ci: 46, ni: 60, mp: 1, mv: 98, mvi: 2823481235114477192
clk:9, ip: 10, ci: 60, ni: 45, mp: 1, mv: 98, mvi: 2823481235114477192
clk:10, ip: 11, ci: 45, ni: 93, mp: 0, mv: 2, mvi: 9223372034707292161
clk:11, ip: 12, ci: 93, ni: 7, mp: 0, mv: 1, mvi: 1
clk:12, ip: 7, ci: 62, ni: 43, mp: 0, mv: 1, mvi: 1
clk:13, ip: 8, ci: 43, ni: 46, mp: 1, mv: 98, mvi: 2823481235114477192
clk:14, ip: 9, ci: 46, ni: 60, mp: 1, mv: 99, mvi: 7080568430684385901
clk:15, ip: 10, ci: 60, ni: 45, mp: 1, mv: 99, mvi: 7080568430684385901
clk:16, ip: 11, ci: 45, ni: 93, mp: 0, mv: 1, mvi: 1
clk:17, ip: 12, ci: 93, ni: 7, mp: 0, mv: 0, mvi: 0
clk:18, ip: 14, ci: 0, ni: 0, mp: 0, mv: 0, mvi: 0
```

Input:

```sh
a
```

Output:

```sh
b
c
```

### Reference

Idea from:

- https://neptune.cash/learn/brainfuck-tutorial/
- https://aszepieniec.github.io/stark-brainfuck/
- https://github.com/aszepieniec/stark-brainfuck/tree/master

**STARK**

- https://starkware.co/stark-101/
- https://aszepieniec.github.io/stark-anatomy/
- https://github.com/lambdaclass/STARK101-rs

**Brainfuck**

- https://thorstenball.com/blog/2017/01/04/a-virtual-brainfuck-machine-in-go/
