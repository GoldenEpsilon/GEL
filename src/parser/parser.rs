
use std::collections::HashMap;
use crate::datatypes::GrammarToken;
use crate::datatypes::ASTNode;

//A Top-Down Parser
pub fn parser(token_list: Vec<(String, String, i32)>, grammar: HashMap<String, Vec<Vec<GrammarToken>>>) -> ASTNode {
	let none = GrammarToken{is_terminal: true, value: String::from("NONE"), lookahead: vec!["NONE".to_string()], is_subrule: false};
	let end = GrammarToken{is_terminal: true, value: String::from("END"), lookahead: vec!["".to_string()], is_subrule: false};
	let endtoken = (String::from("END"), String::from(""), 0);
	let mut ast = ASTNode{rule: "Root".to_string(), data: None, children: vec![], line: 0};
	let mut ast_focus = vec![];
	let mut ast_stack = vec![];
	let mut focus = &GrammarToken{is_terminal: false, value: String::from("Root"), lookahead: vec!["END".to_string()], is_subrule: false};
	let mut stack = vec![&end];
	let mut tokens = token_list.iter();
	//Check for an empty program
	let first_match = tokens.next();
	if first_match.is_none() {
		return ast;
	}
	let mut to_match = first_match.unwrap();
	
	loop {
		if to_match.0 == end.value && focus.value == none.value {
			//println!("Parsing passed!");
			break;
		} else if focus.value == none.value {
			if stack.len() == 0 {
				panic!("Whoops, parser error!\nRan out of stuff to find when I found a {:#?}, which was {:#?} on line {:#?}", to_match.0, to_match.1, to_match.2);
				//return ast;
			}
			if ast_stack.len() == 0 {
				panic!("Whoops, parser error!\nRan out of stuff in my AST to find when I found a {:#?}, which was {:#?} on line {:#?}", to_match.0, to_match.1, to_match.2);
				//return ast;
			}
			ast_focus = ast_stack.pop().unwrap();
			focus = stack.pop().unwrap();
		} else if focus.is_terminal == false {
			//The rule that will be followed next, the default value should never be used
			let mut rule_out = grammar[&focus.value][0].iter();
			//Make sure there is a rule to follow - if we don't have one this should fail fast
			let mut rule_out_changed = false;
			for rule in &grammar[&focus.value] {
				if rule[0].value == to_match.0 {
					rule_out = rule.iter();
					rule_out_changed = true;
					break;
				}else{
					let mut breakout = false;
					for l in &rule[0].lookahead {
						if to_match.0.eq(l) {
							rule_out = rule.iter();
							rule_out_changed = true;
							breakout = true;
							break;
						}else if none.value.eq(l) && to_match.2 != -1 {
							rule_out = rule.iter();
							rule_out_changed = true;
						}
					}
					if breakout { break; }
				}
			}
			if !rule_out_changed {
				if to_match.2 == -1 {
					let tok = tokens.next();
					if tok != None {
						to_match = tok.unwrap();
					} else {
						to_match = &endtoken;
					}
					continue;
				}
				panic!("Whoops, parser error!\nI ran into a dead end thinking that I found a {:#?}, but I found a {:#?}, which was {:#?} on line {:#?}", focus.value, to_match.0, to_match.1, to_match.2);
				//return ast;
			}
			for r in rule_out.rev() {
				stack.push(r);
				if !r.is_subrule {
					find_ast_node(&ast_focus, &mut ast).children.push(ASTNode{rule: r.value.to_owned(), data: None, children: vec![], line: to_match.2});
				} else {
					find_ast_node(&ast_focus, &mut ast).children.push(ASTNode{rule: "SUBRULE".to_owned(), data: None, children: vec![], line: to_match.2});
				}
			}
			find_ast_node(&ast_focus, &mut ast).children.reverse();
			for i in (0..find_ast_node(&ast_focus, &mut ast).children.len()).rev() {
				let mut ast_path = ast_focus.to_vec();
				ast_path.push(i);
				ast_stack.push(ast_path);
			}
			focus = stack.pop().unwrap();
			ast_focus = ast_stack.pop().unwrap().to_owned();
		} else if focus.value == to_match.0 {
			if stack.len() == 0 {
				panic!("Whoops, parser error!\nRan out of stuff to find when I found a {:#?}, which was {:#?} on line {:#?}", to_match.0, to_match.1, to_match.2);
				//return ast;
			}
			find_ast_node(&ast_focus, &mut ast).data = Some(to_match.to_owned());
			find_ast_node(&ast_focus, &mut ast).line = to_match.2;
			let tok = tokens.next();
			if tok != None {
				to_match = tok.unwrap();
			} else {
				to_match = &endtoken;
			}
			focus = stack.pop().unwrap();
			if ast_stack.len() == 0 {
				panic!("Whoops, parser error!\nRan out of stuff in my AST to find when I found a {:#?}, which was {:#?} on line {:#?}", to_match.0, to_match.1, to_match.2);
				//return ast;
			}
			ast_focus = ast_stack.pop().unwrap();
		} else if to_match.2 == -1 {
			let tok = tokens.next();
			if tok != None {
				to_match = tok.unwrap();
			} else {
				to_match = &endtoken;
			}
		} else {
			panic!("Whoops, parser error!\nI was thinking I would find a {:#?}, but I found a {:#?}, which was {:#?} on line {:#?}", focus.value, to_match.0, to_match.1, to_match.2);
			//return ast;
		}
	}

	//Remove subrules to clean up the AST
	clean_ast(&mut ast);

	return ast;
}

fn clean_ast(ast: &mut ASTNode) {
	let mut indices = vec![];
	for (index, node) in &mut ast.children.iter_mut().enumerate() {
		if node.rule != "SUBRULE".to_owned() {
			clean_ast(node);
		} else {
			indices.push(index);
		}
	}
	let mut offset = 0;
	for index in &indices {
		let len = ast.children.len();
		ast.children.splice((index+offset)..(index+offset+1), ast.children[*index].children.to_owned());
		offset += len - 1;
	}
	if indices.len() > 0 {
		clean_ast(ast);
	}
}

fn find_ast_node<'a>(path: &Vec<usize>, ast: &'a mut ASTNode) -> &'a mut ASTNode {
	if path.len() == 0 {
		return ast;
	}
	return find_ast_node_2(path, &mut ast.children[path[0]], 1);
}

fn find_ast_node_2<'a>(path: &Vec<usize>, ast: &'a mut ASTNode, i: usize) -> &'a mut ASTNode {
	if i < path.len() {
		return find_ast_node_2(path, &mut ast.children[path[i]], i+1);
	}else{
		return ast;
	}
}