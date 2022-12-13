use regex::Regex;
use std::collections::HashMap;
use crate::datatypes::GrammarToken;

//A Converter from an input CFG to what the Top-Down Parser needs,
//currently requires a predictive CFG
	//ideas for implementation:
		//check lookahead groups to see if two subrules give some of the same tokens on a lookahead group
			//if so, mark it for later
			//if a subrule has a subset of the tokens of another subrule, split the rule and merge the subrules
			//if a rule is marked and no rules were split in a pass, error.
			//otherwise, if a rule is marked rerun the backtrack fixer
			//otherwise, the grammar SHOULD be fixed
			//Print out the issues the grammar had so that the dev can fix it
		//If a grammar is not predictive, fix it while keeping associativity
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
				rule.push(GrammarToken{is_terminal: token.to_uppercase().eq(token), value: token.to_string(), lookahead: vec!["NONE".to_string()]});
			}
			rule_list.push(rule);
		}
		//insert as new GrammarToken lists (grammar.insert();)
		grammar.insert(g.get(1).unwrap().as_str().to_string(), rule_list);
	}
	//now find the look-ahead groups
	let mut return_grammar: HashMap<String, Vec<Vec<GrammarToken>>> = HashMap::new();
	for g in &grammar {
		let mut rule_list = vec![];
		for r in g.1 {
			let mut rule = vec![];
			for tok in r {
				let look_ahead = find_lookahead(tok, &grammar);
				rule.push(GrammarToken{is_terminal: tok.is_terminal, value: tok.value.clone(), lookahead: look_ahead});
			}
			rule_list.push(rule);
		}
		return_grammar.insert(g.0.to_string(), rule_list);
	}
	for g in &return_grammar {
		let mut i1 = 1;
		for r in g.1 {
			let mut i2 = 1;
			for r2 in g.1 {
				if i1 != i2 && r2[0].lookahead.iter().all(|item| item != "NONE" && r[0].lookahead.contains(item)) {
					panic!("rules {} and {} of {} stop backtracking from working! Please fix now.", i1, i2, g.0);
				}
				i2+=1;
			}
			i1+=1;
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