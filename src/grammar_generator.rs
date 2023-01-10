use regex::Regex;
use std::collections::HashMap;
use crate::datatypes::GrammarToken;

//A Converter from an input CFG to what the Top-Down Parser needs.
//Predictive grammar is not needed, as the generator handles that case.
//TODO:
//		Make check and fix for left recursion (similar to how it fixes non-predictive grammars)
//		Make optional tokens/linked optional tokens (signified with (x) beforehand, with x being a tag)
//			for example, Test::= (parens)LPAREN ID (parens)RPAREN (semicolon)SEMI
//	There are two special values, being grammar rule Root and the the token type END.
//	END is used for the end of the program, for if you want to end parsing early.
//	Root is always the highest-level set of rules. (the lowest priority operators)
//	Root is the only special value you need to use, the others are just there for simplification.
pub fn grammar_generator(inputstr: String) -> HashMap<String, Vec<Vec<GrammarToken>>>{
	let mut grammar: HashMap<String, Vec<Vec<GrammarToken>>> = HashMap::new();
	//make a for loop to iterate through all parts of the cfg
	for g in Regex::new(r"(?m)\s*(\w*)\s*::=\s*(.*)\s*$").unwrap().captures_iter(&inputstr) {
		let mut rule_list = vec![];
		//make the list of options (split by | then parse each)
		for r in Regex::new(r"(\s*\w*\s*)*(\||$)").unwrap().find_iter(g.get(2).unwrap().as_str()) {
			let mut rule = vec![];
			for tok in Regex::new(r"\s*(\w+)\s*").unwrap().captures_iter(r.as_str()) {
				let token = tok.get(1).unwrap().as_str();
				rule.push(GrammarToken{is_terminal: token.to_uppercase().eq(token), value: token.to_string(), lookahead: vec!["NONE".to_string()], is_subrule: false});
			}
			rule_list.push(rule);
		}
		//insert as new GrammarToken lists (grammar.insert();)
		grammar.insert(g.get(1).unwrap().as_str().to_string(), rule_list);
	}
	
	let mut return_grammar: HashMap<String, Vec<Vec<GrammarToken>>>;
	loop {
		//now find the look-ahead groups and fix issues with them
		return_grammar = HashMap::new();
		for g in &grammar {
			let mut rule_list = vec![];
			for r in g.1 {
				let mut rule = vec![];
				for tok in r {
					let look_ahead = find_lookahead(tok, &grammar);
					rule.push(GrammarToken{is_terminal: tok.is_terminal, value: tok.value.clone(), lookahead: look_ahead, is_subrule: tok.is_subrule});
				}
				rule_list.push(rule);
			}
			return_grammar.insert(g.0.to_string(), rule_list);
		}
		let mut rule_to_fix = (false, "", 0, 0);
		'outer: for g in &return_grammar {
			let mut i1 = 1;
			for r in g.1 {
				let mut i2 = 1;
				for r2 in g.1 {
					if i1 != i2 && r2[0].lookahead.iter().all(|item| item != "NONE" && r[0].lookahead.contains(item)) {
						rule_to_fix = (true, g.0, i1 - 1, i2 - 1);
						break 'outer;
						//panic!("rules {} and {} of {} stop backtracking from working! Please fix now.", i1, i2, g.0);
					}
					i2+=1;
				}
				i1+=1;
			}
		}
		//we found an issue! Time to fix it up
		if rule_to_fix.0 {
			//add first element (which should be the same for each) to new subrule, move other elements to new rule as an "or"
			//println!("{:?}, {:?}, {:?}", rule_to_fix.1, rule_to_fix.2, rule_to_fix.3);
			//println!("{:?}", grammar[rule_to_fix.1]);
			//println!("{:?}, {:?}", grammar[rule_to_fix.1][rule_to_fix.2], grammar[rule_to_fix.1][rule_to_fix.3]);
			if grammar[rule_to_fix.1][rule_to_fix.2][0].value == grammar[rule_to_fix.1][rule_to_fix.3][0].value {
				let mut subrule_num = 2;
				let mut subrule_name = format!("{}_{}", rule_to_fix.1, subrule_num);
				while grammar.contains_key(&subrule_name) {
					subrule_num += 1;
					subrule_name = format!("{}_{}", rule_to_fix.1, subrule_num);
				}
				let mut rule = grammar[rule_to_fix.1].to_owned();
				rule.push(vec![rule[rule_to_fix.2][0].to_owned(), GrammarToken{is_terminal: false, value: subrule_name.to_string(), lookahead: vec!["NONE".to_string()], is_subrule: true}]);
				rule[rule_to_fix.2].remove(0);
				rule[rule_to_fix.3].remove(0);
				let mut new_rule = vec![rule[rule_to_fix.2].to_owned(), rule[rule_to_fix.3].to_owned()];
				if new_rule[0].len() == 0 {
					new_rule[0].push(GrammarToken{is_terminal: true, value: "NONE".to_string(), lookahead: vec!["NONE".to_string()], is_subrule: false});
				}
				if new_rule[1].len() == 0 {
					new_rule[1].push(GrammarToken{is_terminal: true, value: "NONE".to_string(), lookahead: vec!["NONE".to_string()], is_subrule: false});
				}
				grammar.insert(subrule_name.to_string(), new_rule);
				rule.remove(rule_to_fix.2);
				if rule_to_fix.2 <= rule_to_fix.3 {
					rule_to_fix.3 -= 1;
				}
				rule.remove(rule_to_fix.3);
				grammar.remove(&rule_to_fix.1.to_string());
				grammar.insert(rule_to_fix.1.to_string(), rule);
			}else{
				//if they aren't equal that means that there are nonterminals involved here, making things trickier.
				//Problem: what to do with:
				// A: B C | D C
				// B: C | b
				// C: a c
				// D: a d
				// (problem being that you have to go through B in code, but can't let it diverge from going to C)
				panic!("This grammar is not predictive! Take another look at {} and its descendants", rule_to_fix.1);
			}
		} else {
			break;
		}
	}
	return return_grammar;
}

fn find_lookahead(token: &GrammarToken, grammar: &HashMap<String, Vec<Vec<GrammarToken>>>) -> Vec<String>{
	let mut ret_val = vec![];
	if !token.is_terminal {
		for rule in &grammar[&token.value] {
			ret_val.append(&mut find_lookahead(&rule[0], &grammar));
		}
	}else{
		return vec![token.value.clone()];
	}
	return ret_val;
}