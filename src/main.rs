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
use macroquad::ui::InputHandler;
use macroquad::ui::Skin;
use macroquad::ui::root_ui;
use macroquad::ui::hash;
use macroquad::ui::widgets;
use regex::Regex;
use crate::parser::parser_setup::*;
use crate::interpreter::interpreter::interpret_program;

mod datatypes;
mod parser;
mod scanner;
mod optimizers;
mod interpreter;
mod grammar_generator;

#[macroquad::main("BasicShapes")]
async fn main() {
    let console_skin = {
        let editbox_style = root_ui()
            .style_builder()
            .font_size(30)
            .background(Image::gen_image_color(1, 1, Color::from_rgba(40, 40, 40, 255)))
            .text_color(Color::from_rgba(255, 255, 255, 255))
            .build();

        Skin {
            editbox_style,
            ..root_ui().default_skin()
        }
    };
    std::env::set_var("RUST_BACKTRACE", "1");
	println!("0");
    let filename;
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        filename = "";
	}else{
        filename = &args[1];
    }
	let mut program = compile_file(filename);
	println!("1");
	interpret_program(&mut program, "");
	interpret_program(&mut program, "init");
    let mut console = false;
    let mut consoletext = "".to_owned();
    let mut copy = "".to_owned();
    let mut current_frame = 0;
    loop {
        /*clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);*/

        if console {
            let mut nextchar = get_char_pressed();
            while nextchar.is_some() {
                //println!("{}", nextchar.unwrap() as u32);
                //24=ctrlx,3=ctrlc,22=ctrlv,8=bckspc,13=enter
                match nextchar.unwrap() {
                    /*'\u{00061}' => {
                        //let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();
                        //let msg = consoletext.as_str();
                        //ctx.set_contents(msg.to_owned()).unwrap();
                        //println!("{}", ctx.get_contents().unwrap());
                    }
                    '\u{0008}' => {
                        consoletext.pop();
                    }
                    '\u{0003}' => {
                        copy = consoletext.to_owned();
                    }
                    '\u{0018}' => {
                        copy = consoletext.to_owned();
                        consoletext = "".to_owned();
                    }
                    '\u{0016}' => {
                        //let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();
                        //consoletext = format!("{}{}", consoletext, ctx.get_contents().unwrap());
                    }*/
                    '\u{000d}' => {
                        if let Some(captures) = Regex::new(r"/gel (.*)").unwrap().captures(&consoletext) {
                            interpret_program(&mut compile(captures.get(1).unwrap().as_str().to_owned()), "");
                            consoletext = "".to_owned();
                        }
                    }
                    _ => {}
                }
                nextchar = get_char_pressed();
            }
        }

	    interpret_program(&mut program, "step");
        
        let console_opening = !console;
        if is_key_pressed(KeyCode::GraveAccent) {
            console = !console;
            root_ui().mouse_down((10.0, screen_height() - 50.0));
        }

	    interpret_program(&mut program, "draw");

        if console {
            
            root_ui().push_skin(&console_skin);

            widgets::InputText::new(hash!()).size(vec2(screen_width(), 30.0)).position(vec2(0.0, screen_height() - 60.0)).ui(&mut root_ui(), &mut consoletext);
            
            if console_opening {
                consoletext = "".to_owned();
            }
            root_ui().pop_skin();
        }

        current_frame += 1;
        next_frame().await;
        if copy.len() > 0 {
            //let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();
            //let _ = ctx.set_contents(copy);
        }
        copy = "".to_owned();
    }
}