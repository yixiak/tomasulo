# Tomasulo Simulator

This is a simulator for Speculative Tomasulo algorithm.

# Detail

The project is based on the following principles：
- There is 1 FP Adder and 1 FP Multipler
- In reservation station，there are 3 Load，3 Store，2 fadd and 2 fmult.
- Record Buffer has 8 entries.
- the instruction will begin execute 1 cycle after others write back the required result.

|Instruction|Ex cycle|
|---|---|
|Load|2|
|Store|2|
|Fadd|2|
|Fmult|10|
|Fdiv|20|

## Test Instruction

```c
// input1
LD F6 34+ R2
LD F2 45+ R3
MULTD 0 F2 F4
SUBD F8 F6 F2
DIVD F10 F0 F6
ADDD F6 F8 F2
```
```c
// input 2
LD F2 0 R2
LD F4 0 R3
DIVD F0 F4 F2
MULTD F6 F0 F2
ADDD F0 F4 F2
SD F6 0 R3
MULTD F6 F0 F2
SD F6 0 R1

```

# Reference
[GZTimeWalker/tomasulo-sim](https://github.com/GZTimeWalker/tomasulo-sim)

[计算机体系结构-重排序缓存ROB](https://zhuanlan.zhihu.com/p/501631371)