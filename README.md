# Your task is to implement a Brainfuck interpreter

Brainfuck’s is composed of only eight simple commands, a data pointer and an instruction pointer. This [wikipedia](https://en.wikipedia.org/wiki/Brainfuck) reference is enough to get started with the language specification.

While not listed in requirements, testing is advised.

## Requirements
- Make it stack-based. This should mean that every loop must be in stack one level below current. For example for script ‘+[-]-’ it could mean that operations would be as such: “add +1, run loop, sub 1”, where “run loop” includes necessary operations and actions inside.
- The "," command will not be a required for the implementation.
- Nested loops must be supported.
- Make it read input from a read stream implementation for your preferred language without knowing all input at once. This requirement excludes look-ahead. For example to find closing brackets for loops. As such, the next token should be read and processed only after the current one is finished being processed.
Loops preparation might be using the same process to fill them. This means whilst creating a loop – it can read necessary tokens from the input, while still adhering to this rule.

## How to run

Assuming you have Rust installed in your system.

Testing:
```sh
cargo test
```

Run hello-worl example:
```sh
cargo run "./bf_samples/hello_world.bf"
```

Run `2 + 5` example:
```sh
cargo run "./bf_samples/add_2_5.bf"
```
