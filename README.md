Rust application that generates the Kolakoski sequence.

Verified with the sequence at [WolframAlpha](https://www.wolframalpha.com/input/?i=kolakoski+sequence).

## Usage

```bash
cargo run N
```

with N equal to the number of elements to generate.

## What is the Kolakoski sequence?

The Kolakoski sequence is a mathematical sequence composed of only 1s and 2s. The first two numbers are 1 and 2. The following numbers are chosen this way: let's call a **run** a series of identical numbers (all 1s or all 2s) in the sequence.

If we look at the length of each run in the Kolakoski sequence, and write the lengths one after the other, we obtain...the Kolakoski sequence.

Let's take the first 10 numbers of the sequence:

```
1 2 2 1 1 2 1 2 2 1
```

So we have one 1, followed by two 2s, two 1s, then one 2...

```
1 2 2 1...
```

and so on and so forth.
