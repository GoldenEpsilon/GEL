use crate::datatypes::TokenAction;

pub fn token_actions(token: (String, String, i32), action: TokenAction, line_counter: &mut i32, new_line: bool, whitespace_tracker: &mut Vec<usize>) -> Vec<(String, String, i32)>{
	let mut ret_val = vec![];
	if new_line {
		match action {
			TokenAction::Whitespace => {
				let length = token.1.len();
				while whitespace_tracker.len() > 0 {
					if length > *whitespace_tracker.last().unwrap() {
						ret_val.push(("INDENT".to_string(), token.1.to_owned(), -1));
						whitespace_tracker.push(length);
						break;
					}else if length < *whitespace_tracker.last().unwrap() {
						ret_val.push(("DEDENT".to_string(), token.1.to_owned(), -1));
						whitespace_tracker.pop();
					}
				}
			},
			_ => {
				while whitespace_tracker.len() > 0 {
					ret_val.push(("DEDENT".to_string(), token.1.to_owned(), -1));
					whitespace_tracker.pop();
				}
			}
		}
	}
	match action {
		TokenAction::Identity => ret_val.push(token),
		TokenAction::Keywords => ret_val.push(token),
		TokenAction::Bool => 
			match token.0.as_str() {
				"TRUE" => ret_val.push(("NUM".to_string(), "1".to_string(), token.2)),
				"FALSE" => ret_val.push(("NUM".to_string(), "0".to_string(), token.2)),
				_ => {}
			},
		TokenAction::Comment => {},
		TokenAction::Newline => {
			*line_counter = *line_counter + 1;
		},
		TokenAction::Whitespace => {},
		_ => ret_val.push(token)
	}
	return ret_val;
}