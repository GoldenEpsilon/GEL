use regex::Regex;
use once_cell::sync::Lazy;
use crate::datatypes::TokenAction;
use crate::scanner::token_actions::token_actions;

//An Exact Match Scanner
pub fn scanner(inputstr: String, token_list: &Lazy<Vec<(&str, &str, TokenAction)>>) -> Vec<(String, String, i32)> {
	let mut ret_val: Vec<(String, String, i32)> = vec![];
    let mut regex_builder = "".to_owned();
	
	for t in token_list.iter(){
		regex_builder.push_str("(?P<");
		regex_builder.push_str(t.0);
		regex_builder.push_str(">");
		regex_builder.push_str(t.1);
		regex_builder.push_str(")|");
	}
	
	if regex_builder.len() > 0 {
		regex_builder.pop();
	}
	
	//Our iterator of tokens
	let re = Regex::new(&regex_builder).unwrap();
	let tokens = re.captures_iter(inputstr.as_str());
	let mut line = 1;
	let mut new_line = true;
	let mut whitespace_tracker = vec![];
    for tok in tokens {
        for t in token_list.iter() {
			if tok.name(t.0) != None {
				let prev_line = line;
				let mut token = token_actions((t.0.to_owned(), tok.name(t.0).unwrap().as_str().to_owned(), line), t.2.to_owned(), &mut line, new_line, &mut whitespace_tracker);
				if prev_line != line {
					new_line = true;
				} else {
					new_line = false;
				}
				ret_val.append(&mut token);
			}
		}
    }
	
	return ret_val
}