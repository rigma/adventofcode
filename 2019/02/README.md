# Solution to "_Day 2: 1202 Program Alarm_"

## Part 1

On the way to your [gravity assist](https://en.wikipedia.org/wiki/Gravity_assist) around 
the Moon, your ship computer beeps angrily about a "[1202 program alarm](https://www.hq.nasa.gov/alsj/a11/a11.landing.html#1023832)". 
On the radio, an Elf is already explaining how to handle the situation: "Don't worry, that's perfectly norma--" 
The ship computer [bursts into flames](https://en.wikipedia.org/wiki/Halt_and_Catch_Fire).

You notify the Elves that the computer's [magic smoke](https://en.wikipedia.org/wiki/Magic_smoke) 
seems to have escaped. "That computer ran **Intcode** programs like the gravity assist program 
it was working on; surely there are enough spare parts up there to build a new Intcode computer!"

An Intcode program is a list of [integers](https://en.wikipedia.org/wiki/Integer) separated by 
commas (like `1,0,0,3,99`). To run one, start by looking at the first integer (called position `0`). 
Here, you will find an **opcode** - either `1`, `2`, or `99`. The opcode indicates what to do; for example, 
99 means that the program is finished and should immediately halt. Encountering an unknown opcode 
means something went wrong.

Opcode `1` **adds** together numbers read from two positions and stores the result in a third 
position. The three integers **immediately after** the opcode tell you these three positions - the 
first two indicate the **positions** from which you should read the input values, and the third indicates 
the **position** at which the output should be stored.

For example, if your Intcode computer encounters `1,10,20,30`, it should read the values at positions 
`10` and `20`, add those values, and then overwrite the value at position `30` with their sum.

Opcode `2` works exactly like opcode `1`, except it **multiplies** the two inputs instead of adding them. 
Again, the three integers after the opcode indicate **where** the inputs and outputs are, not their values.

Once you're done processing an opcode, **move to the next one** by stepping forward `4` positions.

You have to implement a working emulator that is executing a program wrote in **Intcode**.

Once you have a working computer, the first step is to restore the gravity assist program to the "1202 program alarm" 
state it had just before the last computer caught fire. To do this, **before running the program**, replace position `1` with 
the value `12` and replace position `2` with the value `2`. What value is left at position `0` after the program halts?

[Link to the challenge](https://adventofcode.com/2019/day/2)
[Version of the solution](https://github.com/rigma/adventofcode/tree/609c385eb86cff52c4c0f964faba2acc847cb01b)

## Part 2

"Good, the new computer seems to be working correctly! **Keep it nearby** during this mission - you'll probably use it 
again. Real Intcode computers support many more features than your new one, but we'll let you know what they are as 
you need them."

"However, your current priority should be to complete your gravity assist around the Moon. For this mission to succeed, 
we should settle on some terminology for the parts you've already built."

Intcode programs are given as a list of integers; these values are used as the initial state for the computer's **memory**. 
When you run an Intcode program, make sure to start by initializing memory to the program's values. A position in memory 
is called an **address** (for example, the first value in memory is at "address 0").

Opcodes (like `1`, `2`, or `99`) mark the beginning of an instruction. The values used immediately after an opcode, if any, 
are called the instruction's **parameters**. For example, in the instruction `1,2,3,4`, `1` is the opcode; `2`, `3`, and `4` 
are the parameters. The instruction `99` contains only an opcode and has no parameters.

The address of the current instruction is called the **instruction pointer**; it starts at `0`. After an instruction finishes, 
the instruction pointer increases by **the number of values in the instruction**; until you add more instructions to the computer, 
this is always `4` (`1` opcode + `3` parameters) for the add and multiply instructions. (The halt instruction would increase the 
instruction pointer by `1`, but it halts the program instead.)

"With terminology out of the way, we're ready to proceed. To complete the gravity assist, you need to determine what pair of 
inputs produces the output `19690720`."

The inputs should still be provided to the program by replacing the values at addresses `1` and `2`, just like before. In this program, 
the value placed in address `1` is called the **noun**, and the value placed in address `2` is called the **verb**. Each of the two input values 
will be between `0` and `99`, inclusive.

Once the program has halted, its output is available at address `0`, also just like before. Each time you try a pair of inputs, make 
sure you first **reset the computer's memory to the values in the program** (your puzzle input) - in other words, don't reuse memory 
from a previous attempt.

Find the input **noun** and **verb** that cause the program to produce the output `19690720`. What is `100 * noun + verb`? (For example, if 
`noun=12` and `verb=2`, the answer would be `1202`.)

[Link to the challenge](https://adventofcode.com/2019/day/2#part2)
[Version of the solution](https://github.com/rigma/adventofcode/tree/178f9053d7e296a5be6ee795d5864178c334207a)

# Implementation

Again, I've chose to use Rust to implement this challenge, because why not? Futhermore, I thought the language was more fitted to this
challenge because the main objective is to implement a litte emulator of an imaginary CPU. So, in a roleplay perspective, I think that
a system programming language is more likely to be chose to implement such application.
