use std::collections::HashMap;
use crate::{datatypes::{Data, Program, SpriteData}, interpreter::unwrap_values::get_value};
use macroquad::prelude::{Color, camera::mouse};
use regex::Regex;
use rust_decimal::prelude::*;
use macroquad::prelude::*;

struct Function {
    name: &'static str,
    desc: &'static str,
    args: &'static [FunctionArg]
}

struct FunctionArg {
    name: &'static str,
    typ: &'static str
}

const FUNCTION_LIST: &'static [Function] = &[
    Function{name: "print", desc: "Prints {value} into the console (if given more than one argument prints all of them) [Alternative names: trace]", args: &[FunctionArg{name: "value", typ: "any"}]},
    Function{name: "trace", desc: "Prints {value} into the console (if given more than one argument prints all of them) [Alternative names: print]", args: &[FunctionArg{name: "value", typ: "any"}]},
    Function{name: "regex", desc: "Applies the regex {regex} to the string {input} and returns true if a match is found", args: &[FunctionArg{name: "regex", typ: "string"}, FunctionArg{name: "input", typ: "string"}]},
    Function{name: "clear_background", desc: "Clears the screen to {color}", args: &[FunctionArg{name: "color", typ: "color"}]},
    Function{name: "draw_text", desc: "Draws the text {text} at {x}, {y}, with font size {font size} and color {color}", args: &[FunctionArg{name: "text", typ: "string"}, FunctionArg{name: "x", typ: "number"}, FunctionArg{name: "y", typ: "number"}, FunctionArg{name: "font size", typ: "number"}, FunctionArg{name: "color", typ: "color"}]},
    Function{name: "object_create", desc: "Creates an Object of type {type}, calling all relevant creation functions.", args: &[FunctionArg{name: "type", typ: "object type (string)"}]},
    Function{name: "add_sprite", desc: "Adds a new sprite from the path {path} and returns a Sprite", args: &[FunctionArg{name: "path", typ: "path (string)"}]},
    Function{name: "add_sprite", desc: "Adds a new sprite from the path {path} and returns a Sprite named {name}", args: &[FunctionArg{name: "path", typ: "path (string)"}, FunctionArg{name: "sprite name", typ: "string"}]},
    Function{name: "draw_sprite", desc: "Draws the sprite named {sprite name} to the screen at {x}, {y}, blended with color {color}", args: &[FunctionArg{name: "sprite name", typ: "sprite name (string)"}, FunctionArg{name: "x", typ: "number"}, FunctionArg{name: "y", typ: "number"}, FunctionArg{name: "color", typ: "color"}]},
];

pub fn run_builtin(name: &str, args: Vec<Data>, registers: &HashMap<u32, Data>, variables: &HashMap<String, (Data, Data)>, program: &mut Program) -> Option<Data> {
    match name {
        "print" | "trace" => {
            for arg in args {
                println!("{}", get_value(&arg, &registers, &variables).to_string());
            }
            return Some(Data::Null);
        }
        "regex" => {
            if let [Data::String(arg), Data::String(arg2)] = &args[..] {
                return Some(Data::Int(Regex::new(&arg).unwrap().is_match(&arg2) as i32));
            } else {
                builtin_error(name, args);
                return Some(Data::Null);
            }
        }
        "clear_background" => {
            if let [Data::Color(r,g,b,a)] = &args[..] {
                clear_background(Color::new(r.to_f32().unwrap(), g.to_f32().unwrap(), b.to_f32().unwrap(), a.to_f32().unwrap()));
                return Some(Data::Null);
            } else {
                builtin_error(name, args);
                return Some(Data::Null);
            }
        }
        "draw_text" => {
            if let [Data::String(str), Data::Decimal(x), Data::Decimal(y), Data::Decimal(fntsz), Data::Color(r,g,b,a)] = &args[..] {
                draw_text(str.as_str(), x.to_f32().unwrap(), y.to_f32().unwrap(), fntsz.to_f32().unwrap(), Color::new(r.to_f32().unwrap(), g.to_f32().unwrap(), b.to_f32().unwrap(), a.to_f32().unwrap()));
                return Some(Data::Null);
            } else {
                builtin_error(name, args);
                return Some(Data::Null);
            }
        }
        "object_create" => {
            if let [Data::String(object_type)] = &args[..] {
                return Some(Data::Object(program.new_object(object_type.to_owned())));
            } else {
                builtin_error(name, args);
                return Some(Data::Null);
            }
        }
        //TODO: make sprite loading nonsync
        "add_sprite" => {
            if let [Data::String(path), Data::String(name)] = &args[..] {
                return Some(Data::String(SpriteData::new(path.to_owned(), name.to_owned(), program)));
            } else {
                builtin_error(name, args);
                return Some(Data::Null);
            }
        }
        "draw_sprite" => {
            if let [Data::String(spr_name), Data::Decimal(x), Data::Decimal(y), Data::Color(r,g,b,a)] = &args[..] {
                draw_texture(&program.sprites[spr_name].texture, x.to_f32().unwrap(), y.to_f32().unwrap(), Color::new(r.to_f32().unwrap(), g.to_f32().unwrap(), b.to_f32().unwrap(), a.to_f32().unwrap()));
                return Some(Data::Null);
            } else {
                builtin_error(name, args);
                return Some(Data::Null);
            }
        }
        /*"mouse_position" => {
            let mouse_pos = mouse_position();
            return Some(Data::Decimal(mouse_pos.0), Data::Decimal(mouse_pos.1));
        }*/
        _ => {
            return None;
        }
    }
}

fn builtin_error(name: &str, args: Vec<Data>){
    panic!("{}, {:?}", name, args);
}