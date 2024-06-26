//Compiler for GEL, or the Generally Easy Language, or the Golden Epsilon Language, or the Generic Everyday Language... you get the point. It's GEL.
//The core idea behind this language is to be as easy to code in as modding a game, for quick prototypes and casual game creation.
//The gimmicks will be kept on the side for the most part, simplifying what people have to learn to use it.
//The ideal is to have working code be roughly equivalent to pseudocode - a similar concept to python without the things I find annoying about python.

//The number on the left says how likely it is that I'll actually implement it, 1 for extremely likely and 5 for "if I get around to it"

//Ideas unique to this language:
//	3 - DOALL loops are a specific statement that, well, specifically does doall loops, and it runs in parallel for you
//	3 - DISTRIBUTION statements let you separate code into multiple code blocks, and each code block runs in parallel
//	4 - have a way to make DOALL, DISTRIBUTION, etc in multithreading along with some sanity checks for it
//	2 - Variables are either statically typed or dynamically typed, depending on whether they are instantiated with var or not (alternative just being int, float, etc)
//		- You can also do var int x = 0; and such
//	1 - All objects can inherit a trait (rust-style) or have a feature by default that gives it an iterator to go over all objects of that type (similar to running with() on an object in NTT)
//	1 - Comes with a built-in debug console ala NTT/Mod Playground. Maybe a library if I want it to feel more like a normal language, but it should feel like a default behavior.
//	2 - Libraries have all kinds of game-engine-like things like collision, drawing, audio, input, etc (take from rust libraries :P)
//	5 - Also data structures. I want there to be lots and lots of data structures that you can just plop in.

//Ideas I want to take:
//	3 - async functions: you put async at the start, and now when you call it it's run asynchronously (possibly with multithreading)
//	2 - (type) for conversions: such a nice part of C#
//	5 - Duck typing (double check this one, it's a case where I like the theory but haven't checked it in practice)
//	3 - Being able to override and set default behavior for classes like how you can in python
//	2 - Take NTT's form of with(): it's essentially a for-each loop that you don't need to set a specific i for - self/other covers that
//	3 - Take NTT wait statements for use with DISTRIBUTION
//	3 - Rust's trait inheritance might be a good idea to mess with and copy more in-depth (alternative is having a few preprogrammed traits)

//Random Ideas:
//Indicator to show you want to use either 0-indexing or 1-indexing, so that you don't have to guess (either as something like "using" in C or as an inline operator)


use std::env;
use macroquad::prelude::*;
use crate::datatypes::Console;
use crate::parser::parser_setup::*;
use crate::interpreter::interpreter::interpret_program;
use crate::console::*;

mod datatypes;
mod parser;
mod scanner;
mod optimizers;
mod interpreter;
mod grammar_generator;
mod console;

#[macroquad::main("GEL")]
async fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut programs = vec![];
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        programs.push(compile_file(&args[1]));
    }
    let mut copy = String::new();
    let mut current_frame = 0;
    let mut console = Console { open: false, just_opened: false, console_text: String::new(), console_log: vec![], console_history: vec![], index: 0 };
    loop {
        if console.open {
            if is_key_pressed(KeyCode::Tab) {
                console_autocomplete(&mut console, &mut programs);
            }
            if is_key_pressed(KeyCode::Enter) {
                console_submit(&mut console, &mut programs);
            }
        }

        for program in &mut programs {
            program.current_frame = current_frame;
            if !program.initialized {
                if let Err(err) = interpret_program(program, "") {console.console_log.push((format!("Error: {}", err), 600));}
                if let Err(err) = interpret_program(program, "init") {console.console_log.push((format!("Error: {}", err), 600));}
                program.initialized = true;
            }
        }

        for program in &mut programs {
	        if let Err(err) = interpret_program(program, "step") {console.console_log.push((format!("Error: {}", err), 600));}
        }

        for program in &mut programs {
	        if let Err(err) = interpret_program(program, "draw") {console.console_log.push((format!("Error: {}", err), 600));}
        }

        for program in &mut programs {
            console_log(&mut console, program);
        }
        
        console_step(&mut console);

        current_frame += 1;
        next_frame().await;
        if copy.len() > 0 {
            //let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();
            //let _ = ctx.set_contents(copy);
        }
        copy = "".to_owned();
    }
}

#[test]
fn unittest1(){
    let mut program = compile_file("test_files/unittest1.gel");
    println!("{:#?}", program);
    if let Err(err) = interpret_program(&mut program, "init") {println!("{}", format!("Error: {}", err));}
    for message in &program.log {
        println!("{}", message.to_owned());
    }
    assert!(program.log == vec!["1: this should print", "3: this should print"])
}

#[test]
fn unittest2(){
    let mut program = compile_file("test_files/unittest2.gel");
    println!("{:#?}", program);
    if let Err(err) = interpret_program(&mut program, "init") {println!("{}", format!("Error: {}", err));}
    for message in &program.log {
        println!("{}", message.to_owned());
    }
    assert!(program.log == vec!["1", "0"])
}