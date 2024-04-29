
use macroquad::prelude::*;
use macroquad::ui::InputHandler;
use macroquad::ui::Skin;
use macroquad::ui::root_ui;
use macroquad::ui::hash;
use macroquad::ui::widgets;
use regex::Regex;

use crate::datatypes::Console;
use crate::datatypes::Program;
use crate::interpreter::interpreter::interpret_program;
use crate::parser::parser_setup::compile;
use crate::parser::parser_setup::compile_file;



pub fn console_autocomplete(console: &mut Console, programs: &mut Vec<Program>){
    if let Some(captures) = Regex::new(r"/\S*").unwrap().captures(&console.console_text) {
        //go through list of commands that match text (maybe with regex?), if there's a perfect match go to the next alphabetically
    }else if let Some(captures) = Regex::new(r"/(load) (.*)").unwrap().captures(&console.console_text) {
        //autocomplete directories and files, in the same way commands get autocompleted
    }
}

pub fn console_submit(console: &mut Console, programs: &mut Vec<Program>){
    //look, my muscle memory from NTT isn't going away anytime soon, I might as well accept it.
    if let Some(captures) = Regex::new(r"/(gel|run|gml) (.*)").unwrap().captures(&console.console_text) {
        let mut command_program = compile(captures.get(2).unwrap().as_str().to_owned());
        match interpret_program(&mut command_program, "") {
            Err(msg) => {
                console.console_log.push((format!("Error: {}", msg.to_string()), 600));
            }
            _ => {
                for message in &command_program.log {
                    console.console_log.push((message.to_owned(), 600));
                }
            }
        }
    }
    else if let Some(captures) = Regex::new(r"/(load) (.*)").unwrap().captures(&console.console_text) {
        console.console_history.push(console.console_text.to_owned());
        //TODO: if the program already exists, replace instead of loading a new copy
        programs.push(compile_file(captures.get(2).unwrap().as_str()));
    }
    else{
        console.console_log.push((console.console_text.to_owned(), 600));
    }
    console.console_history.push(console.console_text.to_owned());
    console.console_text = String::new();
    console.open = false;
}

pub fn console_log(console: &mut Console, program: &mut Program){
    for message in &program.log {
        console.console_log.push((message.to_owned(), 600));
    }
    program.log = vec![];
}

pub fn console_step(console: &mut Console){
    let console_background_color: Color = Color::from_rgba(40, 40, 40, 255);
    let console_text_color: Color = Color::from_rgba(255, 255, 255, 255);
    let console_skin: Skin = {
        let editbox_style = root_ui()
            .style_builder()
            .font_size(30)
            .background(Image::gen_image_color(1, 1, console_background_color))
            .text_color(console_text_color)
            .build();
    
        Skin {
            editbox_style,
            ..root_ui().default_skin()
        }
    };

    if console.just_opened {
        console.open = true;
        console.index = 0;
        root_ui().mouse_down((screen_width() - 10.0, screen_height() - 50.0));
        console.just_opened = false;
    }

    let mut set_console = "";
    if console.open {
        if is_key_pressed(KeyCode::GraveAccent) {
            console.open = false;
        }
        if is_key_pressed(KeyCode::Escape) {
            console.open = false;
        }
        if is_key_pressed(KeyCode::Up) && console.index < console.console_history.len() {
            console.index += 1;
            console.console_text = console.console_history[console.console_history.len() - console.index].to_owned();
            root_ui().mouse_down((screen_width() - 10.0, screen_height() - 50.0));
        }
        if is_key_pressed(KeyCode::Down) && console.index == 1 {
            console.console_text = String::new();
            console.index -= 1;
        }
        if is_key_pressed(KeyCode::Down) && console.index > 1 {
            console.console_text = console.console_history[console.console_history.len() - console.index].to_owned();
            console.index -= 1;
        }
} else {
        if is_key_pressed(KeyCode::GraveAccent) {
            console.just_opened = true;
            set_console = "/";
        }
        if is_key_pressed(KeyCode::T) {
            console.just_opened = true;
            set_console = "";
        }
        if is_key_pressed(KeyCode::Slash) {
            console.just_opened = true;
            set_console = "/";
        }
    }

    if console.just_opened && console.console_text == "" && console.index == 0 {
        console.console_text = set_console.to_owned();
    }

    if console.open {
        root_ui().push_skin(&console_skin);
        let mut y = screen_height() - 60.0;
        for (text, ref mut age) in console.console_log.iter_mut().rev() {
            if *age > 0 {
                *age = *age - 1;
            }
            let mut lines = vec![];
            for line in text.lines() {
                let mut line_pos = 0;
                while line_pos < line.len() {
                    let mut cut_line = &line[line_pos..];
                    let TextDimensions { width: w, .. } = measure_text(cut_line, None, 30, 1.0);
                    if w > screen_width() - 20.0 {
                        let mut i = line_pos + 1;
                        loop {
                            cut_line = &line[line_pos..i];
                            let TextDimensions { width: w, .. } = measure_text(cut_line, None, 30, 1.0);
                            i += 1;
                            if i >= line.len() || w > screen_width() - 20.0 {
                                i -= 1;
                                line_pos = i;
                                break;
                            }
                        }
                    } else {
                        line_pos = line.len();
                    }
                    lines.push(cut_line.to_owned());
                }
            }
            y -= 5.0;
            for line in lines.iter().rev() {
                y -= 30.0;
                let TextDimensions { width: w, .. } = measure_text(line, None, 30, 1.0);
                draw_rectangle(0.0, y, f32::min(w + 10.0, screen_width()), 30.0, console_background_color);
                draw_text(line, 5.0, y + 25.0, 30.0, console_text_color);
            }
        }
        widgets::InputText::new(hash!()).size(vec2(screen_width(), 30.0)).position(vec2(0.0, screen_height() - 60.0)).ui(&mut root_ui(), &mut console.console_text);
        root_ui().pop_skin();
    } else {
        root_ui().push_skin(&console_skin);
        let mut y = screen_height() - 60.0;
        for (text, ref mut age) in console.console_log.iter_mut().rev(){
            if *age > 0 {
                let mut background_color = console_background_color.to_owned();
                background_color.a = (*age as f32) / 200.0;
                let mut text_color = console_text_color.to_owned();
                text_color.a = (*age as f32) / 200.0;
                let mut lines = vec![];
                for line in text.lines() {
                    let mut line_pos = 0;
                    while line_pos < line.len() {
                        let mut cut_line = &line[line_pos..];
                        let TextDimensions { width: w, .. } = measure_text(cut_line, None, 30, 1.0);
                        if w > screen_width() - 20.0 {
                            let mut i = line_pos + 1;
                            loop {
                                cut_line = &line[line_pos..i];
                                let TextDimensions { width: w, .. } = measure_text(cut_line, None, 30, 1.0);
                                i += 1;
                                if i >= line.len() || w > screen_width() - 20.0 {
                                    i -= 1;
                                    line_pos = i;
                                    break;
                                }
                            }
                        } else {
                            line_pos = line.len();
                        }
                        lines.push(cut_line.to_owned());
                    }
                }
                y -= 5.0;
                for line in lines.iter().rev() {
                    y -= 30.0;
                    let TextDimensions { width: w, .. } = measure_text(line, None, 30, 1.0);
                    draw_rectangle(0.0, y, f32::min(w + 10.0, screen_width()), 30.0, console_background_color);
                    draw_text(line, 5.0, y + 25.0, 30.0, console_text_color);
                }
                *age = *age - 1;
            }
        }
        root_ui().pop_skin();
    }
}