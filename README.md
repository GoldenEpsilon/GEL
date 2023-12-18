# GEL
The compiler and interpreter for GEL, or the Generally Easy Language, or the Golden Epsilon Language, or the Generic Everyday Language... you get the point. It's GEL.

The core idea behind this language is to be as easy to code in as modding a game, for quick prototypes and casual game creation.

The gimmicks will be kept on the side for the most part, simplifying what people have to learn to use it.

The ideal is to have working code be roughly equivalent to pseudocode - a similar concept to python without the things I find annoying about python.

Currently you can compile the code with `cargo build` and run `gel.exe` (by default found in target/debug) with a file in order to run it!

(For example, from the base directory running `./target/debug/gel.exe test_files/test.gel` will run `test.gel`.)

# The Language

Currently there is no guide for how to use GEL, which is intentional as currently I do not think GEL is even turing complete; It's unfinished to the point that the only real function is `print`.

This will change as time goes on, but for now you can see the basics of what the language will be capable of from the test files.
