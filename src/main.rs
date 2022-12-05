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



use regex::Regex;
use std::env;
use std::fs;
use std::fmt;
use std::collections::HashMap;


fn main() {
    let args: Vec<String> = env::args().collect();
	
	let filename = &args[1];
	
	let input = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
	
	let token_list = vec![
		("COMMENT",    r"(//.*)|(/\*(.|\n|\r)*?\*/)|(#.*)", TokenAction::Comment),
		("STRING",    "(\".*?\")|('.*?')|(`.*?`)", TokenAction::Identity),
		("IF",     r"if", TokenAction::Identity),
		("ELSE",     r"else", TokenAction::Identity),
		("FOR",     r"for", TokenAction::Identity),
		("TYPE",    r"int|float|string|var|#define|function|fn", TokenAction::Identity),
		("TRUE",    r"true|True|TRUE", TokenAction::Bool),
		("FALSE",    r"false|False|FALSE", TokenAction::Bool),
		("AND",    r"&&|and", TokenAction::Identity),
		("OR",    r"\|\||or", TokenAction::Identity),
		("EQ",    r"==", TokenAction::Identity),
		("LE",    r"<=", TokenAction::Identity),
		("GE",    r">=", TokenAction::Identity),
		("LT",    r"<", TokenAction::Identity),
		("GT",    r">", TokenAction::Identity),
		("INCR",    r"\+\+", TokenAction::Identity),
		("DECR",    r"--", TokenAction::Identity),
		("SETADD",    r"\+=", TokenAction::Identity),
		("SETSUB",    r"-=", TokenAction::Identity),
		("SETMUL",    r"\*=", TokenAction::Identity),
		("SETDIV",    r"/=", TokenAction::Identity),
		("EXP",    r"\*\*", TokenAction::Identity),
		("DOT",    r"\.", TokenAction::Identity),
		("PLUS",    r"\+", TokenAction::Identity),
		("MINUS",    r"-", TokenAction::Identity),
		("MULT",    r"\*", TokenAction::Identity),
		("DIV",    r"/", TokenAction::Identity),
		("SET",    r"=", TokenAction::Identity),
		("SEMI",    r";", TokenAction::Identity),
		("LPAREN",    r"\(", TokenAction::Identity),
		("RPAREN",    r"\)", TokenAction::Identity),
		("LBRACE",    r"\{", TokenAction::Identity),
		("RBRACE",    r"\}", TokenAction::Identity),
		("ASSIGN",    r"=", TokenAction::Identity),
		("COLON",    r":", TokenAction::Identity),
		("COMMA",    r",", TokenAction::Identity),
		("ID",     r"[a-zA-Z_][a-zA-Z0-9_]*", TokenAction::Keywords),
		("HNUM",    r"0x[0-9a-fA-F]+", TokenAction::HexNum),
		("NUM",    r"-?([0-9]*\.[0-9]+)|[0-9]+", TokenAction::Identity),
		("NEWLINE",    r"\s*?\n", TokenAction::Newline),
		("WHITESPACE",    r"\s*", TokenAction::Whitespace), //Creates INDENT and DEDENT for pythonic whitespace
	];
	//have rules for ID to let function results and stuff like x.y work
	//Exponents are not right-associative
	//Add foreach as an option for for loops
	//Make parentheses optional for for loops
	let cfg = String::from("
		Root::= Block
		Block::= Stat Block | NONE
		Stat::= LBRACE Block RBRACE | COLON PythonBlock | Decl Semi | Stat2 Semi | If | For
		PythonBlock::= INDENT Block DEDENT | Stat
		Stat2::= ID Stat3
		Stat3::= DOT Stat2 | AsgnOp | Func | NONE
		Semi::= SEMI | NONE
		Decl::=	TYPE ID Decl2
		Decl2::= Set Expr | FuncDef | NONE
		AsgnOp::= Set Expr | INCR | DECR
		Set::= SET | SETADD | SETSUB | SETMUL | SETDIV
		Func::= LPAREN Comma RPAREN
		Comma::= Expr Comma | COMMA Expr Comma | NONE
		FuncDef::= LPAREN DefComma RPAREN Block | Block
		DefComma::= Arg DefComma | COMMA Arg DefComma | NONE
		Arg::= TYPE ID ArgDefault | ID ArgDefault
		ArgDefault::= EQ Val | NONE
		If::= IF Expr Stat Else
		Else::=	ELSE Stat |	NONE
		For::= FOR LPAREN Decl SEMI Expr SEMI Stat2 RPAREN Stat
		Expr::=	OpPrec5
		OpPrec5::= OpPrec4 Op5
		Op5::= AND OpPrec4 Op5 | OR OpPrec4 Op5 | NONE
		OpPrec4::= OpPrec3 Op4
		Op4::= EQ OpPrec3 Op4 | LT OpPrec3 Op4 | GT OpPrec3 Op4 | LE OpPrec3 Op4 | GE OpPrec3 Op4 | NONE
		OpPrec3::= OpPrec2 Op3
		Op3::= PLUS OpPrec2 Op3 | MINUS OpPrec2 Op3 | NONE
		OpPrec2::= OpPrec1 Op2
		Op2::= MULT OpPrec1 Op2 | DIV OpPrec1 Op2 | NONE
		OpPrec1::= Unit Op1
		Op1::= EXP Unit Op1 | NONE
		Unit::=	LPAREN Expr RPAREN TypeHint | Stat2 TypeHint | Val TypeHint
		TypeHint::= LPAREN TYPE RPAREN | NONE
		Val::= NUM | STRING
		");
	let grammar = grammar_generator(cfg);
	//println!("{:#?}", grammar);
	let tokens = scanner(input, token_list);
	let ast = parser(tokens, grammar);
	//let mut parse_tree = parser(tokens, grammar);
	//let ast = parse_tree_to_ast(&mut parse_tree, None);
	//println!("{:#?}", ast);
	let mut optimized_ast = optimize_ast(ast);
	//let opcodes = linearize_ast(optimized_ast, linearize as fn(&mut ASTNode, &mut Vec<Opcode>));
	let mut register = 1;
	let opcodes = linearize(&mut optimized_ast, &mut register);
	//println!("{:#?}", opcodes);
	//Make control flow graph?
	//let optimized_opcodes = optimize_opcodes(opcodes);
	interpret_opcodes(opcodes);
}
/*fn parse_tree_to_ast(parse_tree: &mut ASTNode, passed_node: Option<ASTNode>) -> ASTNode{
	match parse_tree.rule.as_str(){
		"Expr" => {
			if parse_tree.children.len() == 1 {
				return parse_tree.clone();
			}else{
				let child0 = Some(parse_tree.children[0].clone());
				return parse_tree_to_ast(&mut parse_tree.children[1], child0);
			}
		}
		"OpPrec1" | "OpPrec2" | "OpPrec3" | "OpPrec4" => {
			if parse_tree.children.len() > 1 {
				let child0 = Some(parse_tree_to_ast(&mut parse_tree.children[0], passed_node));
				return parse_tree_to_ast(&mut parse_tree.children[1], child0);
			}else{
				if passed_node.is_some() {
					return passed_node.unwrap();
				}else {
					return parse_tree.clone();
				}
			}
		}
		"Op1" | "Op2" | "Op3" | "Op4" => {
			if parse_tree.children.len() > 2 {
				if passed_node.is_some() {
					let rule = parse_tree.children[0].rule.to_owned();
					let data = parse_tree.data.to_owned();
					let line = parse_tree.children[0].line;
					let child1 = parse_tree.children[1].clone();
					return parse_tree_to_ast(&mut parse_tree.children[2], Some(ASTNode{rule: rule, data: data, children: vec![passed_node.unwrap(), child1], line: line}));
				}else{
					panic!("Parsing error: Operator was passed an invalid node when converting to AST on line {}", parse_tree.line);
				}
			}else if parse_tree.children.len() == 1 {
				return parse_tree_to_ast(&mut parse_tree.children[0], passed_node);
			}else{
				panic!("Invalid operator used on line {} (is the CFG okay?)", parse_tree.line);
			}
		}
		"NONE" => {
			if passed_node.is_some() {
				return passed_node.unwrap();
			} else {
				return parse_tree.clone();
			}
		}
		_ => {
			for i in &mut parse_tree.children {
				*i = parse_tree_to_ast(i, None);
			}
			return parse_tree.clone();
		}
	}
}*/

fn linearize(ast: &mut ASTNode, curr_reg: &mut i32) -> Vec<Opcode> {
	let mut ret_val: Vec<Opcode> = vec![];
	match ast.rule.as_str(){
		"Root" => {
			for i in &mut ast.children {
				ret_val.append(&mut linearize(i, curr_reg));
			}
		}
		"For" => {
			
			//Make sure the assignment isn't a function
			ret_val.append(&mut linearize(&mut ast.children[2], curr_reg));
			
			//label to goto
			ret_val.push(Opcode{instruction: "FOR_LABEL".to_string(), data: Data::Label(*curr_reg), data2: Data::Null, register: 0, line: 0});
			let label_reg = *curr_reg;
			*curr_reg += 1;
			
			//Block
			ret_val.append(&mut linearize(&mut ast.children[8], curr_reg));
			
			//Loop Check
			ret_val.append(&mut linearize(&mut ast.children[4], curr_reg));
			let reg = Data::Register(ret_val.last().unwrap().register);
			
			//Modify iterator
			ret_val.append(&mut linearize(&mut ast.children[6], curr_reg));
			
			//goto label at end of loop if true
			ret_val.push(Opcode{instruction: "FOR_GOTO".to_string(), data: reg, data2: Data::Label(label_reg), register: 0, line: 0});
		}
		"If" => {
			let if_label_reg = *curr_reg;
			*curr_reg += 1;
			let else_label_reg = *curr_reg;
			*curr_reg += 1;
			ret_val.append(&mut linearize(&mut ast.children[1], curr_reg));
			
			//goto label if false
			ret_val.push(Opcode{instruction: "ELSE_GOTO".to_string(), data: Data::Register(ret_val.last().unwrap().register), data2: Data::Label(else_label_reg), register: 0, line: 0});
			
			ret_val.append(&mut linearize(&mut ast.children[2], curr_reg));
			
			//goto label
			ret_val.push(Opcode{instruction: "IF_GOTO".to_string(), data: Data::Null, data2: Data::Label(if_label_reg), register: 0, line: 0});
			
			//label to goto
			ret_val.push(Opcode{instruction: "ELSE_LABEL".to_string(), data: Data::Label(else_label_reg), data2: Data::Null, register: 0, line: 0});
			
			ret_val.append(&mut linearize(&mut ast.children[3], curr_reg));
			
			//label to goto
			ret_val.push(Opcode{instruction: "IF_LABEL".to_string(), data: Data::Label(if_label_reg), data2: Data::Null, register: 0, line: 0});
		}
		"Else" => {
			if ast.children.len() > 1 {
				ret_val.append(&mut linearize(&mut ast.children[1], curr_reg));
			}
		}
		"Stat" => {
			if ast.children.len() == 3 {
				ret_val.append(&mut linearize(&mut ast.children[1], curr_reg));
			} else if ast.children[0].rule == "COLON" {
				ret_val.append(&mut linearize(&mut ast.children[1], curr_reg));
			} else {
				ret_val.append(&mut linearize(&mut ast.children[0], curr_reg));
			}
		}
		"Stat2" => {
			let mut child0 = linearize(&mut ast.children[0], curr_reg);
			let mut child1 = linearize(&mut ast.children[1], curr_reg);
			if child1.len() > 0 {
				let index = child1.len()-1;
				child1[index].data = child0[0].data.to_owned();
				ret_val.append(&mut child1);
			}else{
				ret_val.append(&mut child0);
			}
		}
		"Func" => {
			ret_val.append(&mut linearize(&mut ast.children[1], curr_reg));
			if ret_val.len() > 0 {
				ret_val.push(Opcode{instruction: "FUNC".to_string(), data: Data::Null, data2: Data::Register(ret_val[ret_val.len() - 1].register), register: *curr_reg, line: 0});
			}else{
				ret_val.push(Opcode{instruction: "FUNC".to_string(), data: Data::Null, data2: Data::Null, register: *curr_reg, line: 0});
			}
			*curr_reg += 1;
		}
		"Comma" => {
			if ast.children.len() == 2 {
				let mut child0 = linearize(&mut ast.children[0], curr_reg);
				let reg_1 = Data::Register(child0[child0.len() - 1].register);
				let mut child1 = linearize(&mut ast.children[1], curr_reg);
				let reg_2;
				if child1.len() > 0 {
					reg_2 = Data::Register(child1[child1.len() - 1].register);
				}else{
					reg_2 = Data::Null;
				}
				ret_val.append(&mut child0);
				ret_val.append(&mut child1);
				ret_val.push(Opcode{instruction: "Comma".to_string(), data: reg_1, data2: reg_2, register: *curr_reg, line: 0});
				*curr_reg += 1;
			} else if ast.children.len() == 3 {
				let mut child1 = linearize(&mut ast.children[1], curr_reg);
				let reg_1 = Data::Register(child1[child1.len() - 1].register);
				let mut child2 = linearize(&mut ast.children[2], curr_reg);
				let reg_2;
				if child2.len() > 0 {
					reg_2 = Data::Register(child2[child2.len() - 1].register);
				}else{
					reg_2 = Data::Null;
				}
				ret_val.append(&mut child1);
				ret_val.append(&mut child2);
				ret_val.push(Opcode{instruction: "Comma".to_string(), data: reg_1, data2: reg_2, register: *curr_reg, line: 0});
				*curr_reg += 1;
			}
		}
		"FuncDef" => {
			ret_val.append(&mut linearize(&mut ast.children[1], curr_reg));
			ret_val.push(Opcode{instruction: "FUNCDEF".to_string(), data: Data::Null, data2: Data::Register(ret_val[ret_val.len() - 1].register), register: *curr_reg, line: 0});
			*curr_reg += 1;
		}
		"DefComma" => {
			if ast.children.len() > 1 {
				let mut child0 = linearize(&mut ast.children[0], curr_reg);
				let reg_0 = Data::Register(child0[child0.len() - 1].register);
				let mut child1 = linearize(&mut ast.children[1], curr_reg);
				let reg_1;
				if child1.len() > 0 {
					reg_1 = Data::Register(child1[child1.len() - 1].register);
				}else{
					reg_1 = Data::Null;
				}
				ret_val.append(&mut child0);
				ret_val.append(&mut child1);
				ret_val.push(Opcode{instruction: "Comma".to_string(), data: reg_0, data2: reg_1, register: *curr_reg, line: 0});
				*curr_reg += 1;
			}
		}
		"ID" => {
			ret_val.push(Opcode{instruction: "ID".to_string(), data: Data::Variable(ast.data.as_ref().unwrap().1.to_owned()), data2: Data::Null, register: *curr_reg, line: 0});
			*curr_reg += 1;
		}
		"NUM" => {
			ret_val.push(Opcode{instruction: "Value".to_string(), data: Data::Int(ast.data.as_ref().unwrap().1.parse::<i32>().unwrap()), data2: Data::Null, register: *curr_reg, line: 0});
			*curr_reg += 1;
		}
		"STRING" => {
			ret_val.push(Opcode{instruction: "Value".to_string(), data: Data::String(ast.data.as_ref().unwrap().1.to_owned()[1..ast.data.as_ref().unwrap().1.len() - 1].to_string()), data2: Data::Null, register: *curr_reg, line: 0});
			*curr_reg += 1;
		}
		"OpPrec1" | "OpPrec2" | "OpPrec3" | "OpPrec4" => {
			let mut child0 = linearize(&mut ast.children[0], curr_reg);
			let reg = Data::Register(child0.last().unwrap().register);
			ret_val.append(&mut child0);
			let mut op_list = linearize(&mut ast.children[1], curr_reg);
			if op_list.len() > 1 {
				let index = op_list.iter().position(|r| r.instruction == "FLAG");
				if index.is_some() {
					op_list.remove(index.unwrap());
					op_list[index.unwrap()].data = reg;
				}
			}
			ret_val.append(&mut op_list);
		}
		"Op1" | "Op2" | "Op3" | "Op4" => {
			if ast.children.len() > 2 {
				let child0 = linearize(&mut ast.children[0], curr_reg);
				let mut child1 = linearize(&mut ast.children[1], curr_reg);
				let mut reg = Data::Register(child1[child1.len() - 1].register);
				ret_val.append(&mut child1);
				//pushing a custom instruction here as an indicator for OpPriority to handle
				let index = ret_val.len();
				ret_val.push(Opcode{instruction: "FLAG".to_string(), data: Data::Null, data2: Data::Null, register: 0, line: 0});
				//create new operator, keep track of register
				ret_val.push(Opcode{instruction: child0[0].instruction.to_owned(), data: Data::Null, data2: reg, register: *curr_reg, line: ast.line});
				reg = Data::Register(*curr_reg);
				*curr_reg += 1;
				let mut child2 = linearize(&mut ast.children[2], curr_reg);
				if child2.iter().position(|r| r.instruction == "FLAG").is_some() {
					//set first op's left side to register
					let _index = child2.iter().position(|r| r.instruction == "FLAG").unwrap();
					child2.remove(_index);
					child2[_index].data = reg;
					ret_val.splice(index..index, child2.splice(.._index, []));
					ret_val.append(&mut child2);
				}
			}else if ast.children.len() == 1 {
				let mut child = linearize(&mut ast.children[0], curr_reg);
				ret_val.append(&mut child);
			}
		}
		"Unit" => {
			if ast.children.len() == 4 {
				ret_val.append(&mut linearize(&mut ast.children[1], curr_reg));
				//maybe do type hint stuff
			}else{
				ret_val.append(&mut linearize(&mut ast.children[0], curr_reg));
				//maybe do type hint stuff
			}
		}
		"Decl" => {
			let mut child2 = linearize(&mut ast.children[2], curr_reg);
			ret_val.push(Opcode{instruction: "Declare".to_string(), data: Data::Type(ast.children[0].data.as_ref().unwrap().1.to_owned()), data2: Data::Variable(ast.children[1].data.as_ref().unwrap().1.to_owned()), register: 0, line: ast.line});
			if child2.len() > 0{
				let reg = Data::Register(child2[child2.len() - 1].register);
				ret_val.append(&mut child2);
				ret_val.push(Opcode{instruction: "Set".to_string(), data: Data::Variable(ast.children[1].data.as_ref().unwrap().1.to_owned()), data2: reg, register: *curr_reg, line: ast.line});
				*curr_reg += 1;
			}
		}
		"Decl2" => {
			if ast.children.len() > 1 {
				ret_val.append(&mut linearize(&mut ast.children[1], curr_reg));
			} else {
				ret_val.append(&mut linearize(&mut ast.children[0], curr_reg));
			}
		}
		"AsgnOp" => {
			if ast.children.len() == 2 {
				ret_val.append(&mut linearize(&mut ast.children[1], curr_reg));
				let reg = Data::Register(ret_val[ret_val.len() - 1].register);
				ret_val.push(Opcode{instruction: "Set".to_string(), data: Data::Null, data2: reg, register: *curr_reg, line: ast.line});
				*curr_reg += 1;
			} else if ast.children.len() > 0 {
				for i in &mut ast.children {
					ret_val.append(&mut linearize(i, curr_reg));
				}
			}
		}
		_ => {
			if ast.data.is_some() {
				ret_val.push(Opcode{instruction: ast.rule.to_owned(), data: Data::String(ast.data.as_ref().unwrap().1.to_owned()), data2: Data::Null, register: *curr_reg, line: ast.line});
				*curr_reg += 1;
			}else{
				if ast.children.len() > 0 {
					//ret_val.push(Opcode{instruction: ast.rule.to_owned(), data: "UNIMPLEMENTED".to_string(), data2: "".to_string(), register: 0, line: ast.line});
					for i in &mut ast.children {
						ret_val.append(&mut linearize(i, curr_reg));
					}
				}
			}
		}
	}
	return ret_val;
}

#[derive(Clone)]
#[derive(Debug)]
struct GrammarToken {
	is_terminal: bool,
	value: String,
	lookahead: Vec<String>
}


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
fn grammar_generator(inputstr: String) -> HashMap<String, Vec<Vec<GrammarToken>>>{
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

#[derive(Clone, Copy)]
enum TokenAction {
	Keywords,
	Identity,
	Bool,
	HexNum,
	Newline,
	Comment,
	Whitespace,
}

fn token_actions(token: (String, String, i32), action: TokenAction, line_counter: &mut i32, new_line: bool, whitespace_tracker: &mut Vec<usize>) -> Vec<(String, String, i32)>{
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

//An Exact Match Scanner
fn scanner(inputstr: String, token_list: Vec<(&str, &str, TokenAction)>) -> Vec<(String, String, i32)> {
	let mut ret_val: Vec<(String, String, i32)> = vec![];
    let mut regex_builder = "".to_owned();
	
	for t in &token_list{
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
        for t in &token_list {
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

//A Top-Down Parser
fn parser(token_list: Vec<(String, String, i32)>, grammar: HashMap<String, Vec<Vec<GrammarToken>>>) -> ASTNode {
	let none = GrammarToken{is_terminal: true, value: String::from("NONE"), lookahead: vec!["NONE".to_string()]};
	let end = GrammarToken{is_terminal: true, value: String::from("END"), lookahead: vec!["".to_string()]};
	let endtoken = (String::from("END"), String::from(""), 0);
	let mut ast = ASTNode{rule: "Root".to_string(), data: None, children: vec![], line: 0};
	let mut ast_focus = vec![];
	let mut ast_stack = vec![];
	let mut focus = &GrammarToken{is_terminal: false, value: String::from("Root"), lookahead: vec!["END".to_string()]};
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
			return ast;
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
				find_ast_node(&ast_focus, &mut ast).children.push(ASTNode{rule: r.value.to_owned(), data: None, children: vec![], line: to_match.2});
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
}

//AST stuff
#[derive(Debug)]
#[derive(Clone)]
struct ASTNode {
	rule: String,
	data: Option<(String, String, i32)>,
	children: Vec<ASTNode>,
	line: i32,
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

fn optimize_ast(ast: ASTNode/*, optimizers: Vec<&dyn FnMut(ASTNode) -> ASTNode>*/) -> ASTNode {
	return ast;
}

#[derive(Debug)]
#[derive(Clone)]
struct Opcode {
	instruction: String,//might want to replace this with something more useful and/or faster, like a direct function reference
	data: Data,
	data2: Data,
	register: i32,
	line: i32,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
pub enum Data {
    Null,
    Int(i32),
    String(String),
    Register(i32),
    Label(i32),
    Variable(String),
    Type(String),
    Comma(Box<Data>, Box<Data>),
}
impl fmt::Display for Data {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
		match self {
			Data::Null => {
				write!(f, "NULL")
			}
			Data::Int(data) => {
				write!(f, "{}", data)
			}
			Data::String(data) => {
				write!(f, "{}", data)
			}
			Data::Register(data) => {
				write!(f, "Register {}", data)
			}
			Data::Label(data) => {
				write!(f, "Label {}", data)
			}
			Data::Variable(data) => {
				write!(f, "variable {}", data)
			}
			Data::Type(data) => {
				write!(f, "type {}", data)
			}
			Data::Comma(data1, data2) => {
				write!(f, "{}, {}", data1.to_string(), data2.to_string())
			}
		}
    }
}

fn data_operation(left: Data, right: Data, op: String) -> Data {
	match (left, right, op.as_str()) {
		(Data::Int(l), Data::Int(r), "PLUS") => {
			return Data::Int(l + r);
		}
		(Data::Int(l), Data::Int(r), "MINUS") => {
			return Data::Int(l - r);
		}
		(Data::Int(l), Data::Int(r), "MULT") => {
			return Data::Int(l * r);
		}
		(Data::Int(l), Data::Int(r), "DIV") => {
			return Data::Int(l / r);
		}
		(Data::Int(l), Data::Int(r), "EXP") => {
			//println!("{}, {}, {}", l, r, l.pow(r as u32));
			return Data::Int(l.pow(r as u32));
		}
		(Data::Int(l), Data::Int(r), "GT") => {
			return Data::Int((l > r) as i32);
		}
		(Data::Int(l), Data::Int(r), "LT") => {
			return Data::Int((l < r) as i32);
		}
		(Data::Int(l), Data::Int(r), "EQ") => {
			return Data::Int((l == r) as i32);
		}
		(Data::Int(l), Data::Int(r), "AND") => {
			return Data::Int((l != 0 && r != 0) as i32);
		}
		(Data::Int(l), Data::Int(r), "OR") => {
			return Data::Int((l != 0 || r != 0) as i32);
		}
		(Data::Int(l), Data::Null, "INCR") => {
			return Data::Int(l + 1);
		}
		_ => {
			return Data::Null;
		}
	}
}

fn get_value(data: &Data, registers: &HashMap<Data, Data>, variables: &HashMap<Data, (Data, Data)>) -> Data{
	match data {
		Data::Register(_) => {
			let reg = registers.get(&data);
			if !reg.is_some() {
				panic!("NONEXISTENT REGISTER ACCESS TRYING TO ACCESS {:?}", data);
			}
			return get_value(&reg.unwrap(), registers, variables);
		}
		Data::Variable(_) => {
			let var = variables.get(&data);
			if !var.is_some() {
				panic!("NONEXISTENT VARIABLE ACCESS TRYING TO ACCESS {:?}", data);
			}
			return get_value(&var.unwrap().1, registers, variables);
		}
		_ => {
			return data.clone();
		}
	};
}

fn unwrap_function_inputs(data: &Data, registers: &HashMap<Data, Data>, variables: &HashMap<Data, (Data, Data)>) -> Vec<Data> {
	let mut ret_val = vec![];
	match data {
		Data::Comma(l, r) => {
			ret_val.append(&mut unwrap_function_inputs(l, registers, variables));
			ret_val.append(&mut unwrap_function_inputs(r, registers, variables));
		}
		Data::Register(_) | Data::Variable(_) => {
			ret_val.append(&mut unwrap_function_inputs(&get_value(data, registers, variables), registers, variables));
		}
		Data::Null => {
		}
		val => {
			ret_val.push(val.clone());
		}
	}
	return ret_val;
}

fn interpret_opcodes(opcodes: Vec<Opcode>){
	let mut registers: HashMap<Data, Data> = HashMap::new();
	let mut labels: HashMap<Data, usize> = HashMap::new();
	let mut variables: HashMap<Data, (Data, Data)> = HashMap::new();
	let mut functions: HashMap<Data, (usize, Vec<Data>)> = HashMap::new();
	let mut jump_point = 1;
	let mut position = 1;
	for op in &opcodes {
		match op.instruction.as_str() {
			"FOR_LABEL" | "IF_LABEL" | "ELSE_LABEL" => {
				labels.insert(op.data.clone(), position, );
			}
			"FUNC_DEF" => {
				functions.insert(op.data.clone(), (position, unwrap_function_inputs(&op.data2, &registers, &variables)));
			}
			_ => {}
		}
		position += 1;
	}
	while jump_point != 0 {
		let iter = opcodes.iter().skip(jump_point-1);
		jump_point = 0;
		for op in iter {
			match op.instruction.as_str() {
				"Declare" => {
					variables.insert(op.data2.clone(), (op.data.clone(), Data::Null));
				}
				"Set" => {
					if variables.contains_key(&op.data) {
						if registers.contains_key(&op.data2) {
							variables.get_mut(&op.data).unwrap().1 = registers.get_mut(&op.data2).unwrap().clone();
						}
					}
				}
				"Value" => {
					registers.insert(Data::Register(op.register), op.data.clone());
				}
				"ID" => {
					registers.insert(Data::Register(op.register), op.data.clone());
				}
				"Comma" => {
					registers.insert(Data::Register(op.register), Data::Comma(Box::new(op.data.clone()), Box::new(op.data2.clone())));
				}
				"FUNC" => {
					//TODO: 
					// - Custom functions
					// - type checking
					// - input checking
					let mut data = vec![op.data.clone()];
					data.append(&mut unwrap_function_inputs(&op.data2, &registers, &variables));
					match &data[..] {
						[Data::Variable(func_name), arg] if func_name.to_string().as_str() == "print" => {
							println!("{}", get_value(arg, &registers, &variables).to_string());
							registers.insert(Data::Register(op.register), Data::Null);
						}
						[Data::Variable(func_name), Data::String(arg), Data::String(arg2)] if func_name.to_string().as_str() == "regex" => {
							registers.insert(Data::Register(op.register), Data::Int(Regex::new(&arg).unwrap().is_match(&arg2) as i32));
						}
						_ => {
							println!("INVALID FUNCTION ON LINE {}", op.line);
						}
					}
				}
				"FUNC_DEF" => {}
				"PLUS" | "MINUS" | "MULT" | "DIV" | "EXP" | "GT" | "LT" | "EQ" | "AND" | "OR" => {
					let left = get_value(&op.data, &registers, &variables);
					let right = get_value(&op.data2, &registers, &variables);
					registers.insert(Data::Register(op.register), data_operation(left, right, op.instruction.clone()));
					//println!("{:?}", registers.get(&Data::Register(op.register)));
				}
				"INCR" => {
					variables.get_mut(&op.data).unwrap().1 = data_operation(get_value(&op.data, &registers, &variables), Data::Null, op.instruction.clone());
				}
				"FOR_LABEL" | "IF_LABEL" | "ELSE_LABEL" => {}
				"IF_GOTO" => {
					if labels.contains_key(&op.data2) {
						jump_point = *labels.get(&op.data2).unwrap();
					}else{
						println!("LABEL {:?} DOES NOT EXIST", op.data);
					}
				}
				"FOR_GOTO" => {
					match get_value(&op.data, &registers, &variables) {
						Data::Null => {}
						Data::Int(i) if i == 0 => {}
						_ => {
							if labels.contains_key(&op.data2) {
								jump_point = *labels.get(&op.data2).unwrap();
							}else{
								println!("LABEL {:?} DOES NOT EXIST", op.data);
							}
						}
					}
				}
				"ELSE_GOTO" => {
					match get_value(&op.data, &registers, &variables) {
						Data::Null => {
							if labels.contains_key(&op.data2) {
								jump_point = *labels.get(&op.data2).unwrap();
							}else{
								println!("LABEL {:?} DOES NOT EXIST", op.data2);
							}
						}
						Data::Int(i) if i == 0 => {
							if labels.contains_key(&op.data2) {
								jump_point = *labels.get(&op.data2).unwrap();
							}else{
								println!("LABEL {:?} DOES NOT EXIST", op.data2);
							}
						}
						_ => {}
					}
				}
				opcode => {
					println!("UNKNOWN OPCODE ON LINE {}. OPCODE IS {}", op.line, opcode);
				}
			}
			if jump_point != 0 {
				break;
			}
		}
	}
	//println!("{}{:#?}", "Registers: ", registers);
	//println!("{}{:#?}", "Variables: ", variables);
}