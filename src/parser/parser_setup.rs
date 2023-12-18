use std::fs;
use crate::datatypes::Program;
use crate::datatypes::TokenAction;
use crate::parser::linearize_ast::linearize_ast;
use crate::parser::parser::parser;
use crate::scanner::scanner::scanner;
use crate::grammar_generator::grammar_generator;
use crate::optimizers::ast_optimizer::optimize_ast;

pub fn compile_file(filename: &str) -> Program {
	
	//Check to make sure we have a file to read
	if filename == "" {
		return Program::new();
	}

	//let filename = &args[1];
	
	let input = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

	return compile(input);
}

pub fn compile(input: String) -> Program {
	
	//FUNCDEF is above COMMENT because otherwise #define would count as a comment
	let token_list = vec![
		("FUNCDEF",    r"#define|function|fn", TokenAction::Identity),
		("COMMENT",    r"(//.*)|(/\*(.|\n|\r)*?\*/)|(#.*)", TokenAction::Comment),
		("STRING",    "(\".*?\")|('.*?')|(`.*?`)", TokenAction::Identity),
		("IF",     r"if", TokenAction::Identity),
		("ELSE",     r"else", TokenAction::Identity),
		("FOR",     r"for", TokenAction::Identity),
		("TYPE",    r"int|float|string|var", TokenAction::Identity),
		("TRUE",    r"true|True|TRUE", TokenAction::Bool),
		("FALSE",    r"false|False|FALSE", TokenAction::Bool),
		("ARROW",    r"->", TokenAction::Identity),
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
		("DECIMAL",    r"-?([0-9]*\.[0-9]+)", TokenAction::Identity),
		("INT",    r"-?[0-9]+", TokenAction::Identity),
		("NEWLINE",    r"\s*?\n", TokenAction::Newline),
		("WHITESPACE",    r"\s*", TokenAction::Whitespace), //Creates INDENT and DEDENT for pythonic whitespace
	];
	//Exponents are not right-associative
	//Add foreach as an option for for loops
	//Make function definitions work without {} (if there isn't an lbrace, break on next function definition)
	let cfg = String::from("
		Root::= Block
		Block::= Stat Block | NONE
		Stat::= LBRACE Block RBRACE | COLON PythonBlock | Def Semi | Stat2 Semi | If | For | FuncDef Semi
		PythonBlock::= INDENT Block DEDENT | Stat
		Stat2::= ID DOT Stat2 | ID AsgnOp | ID Func
		Semi::= SEMI | NONE
		Def::=	TYPE ID Set Expr | TYPE ID
		AsgnOp::= Set Expr | INCR | DECR | NONE
		Set::= SET | SETADD | SETSUB | SETMUL | SETDIV
		Func::= LPAREN Comma RPAREN
		Comma::= Expr Comma | COMMA Expr Comma | NONE
		FuncDef::= FUNCDEF ID FuncDefArgs FuncDefType Stat
		FuncDefArgs::= LPAREN DefComma RPAREN | NONE
		FuncDefType::= ARROW TYPE | NONE
		DefComma::= Arg DefComma | COMMA Arg DefComma | NONE
		Arg::= TYPE ID ArgDefault | ID ArgDefault
		ArgDefault::= EQ Val | NONE
		If::= IF Expr Stat Else
		Else::=	ELSE Stat |	NONE
		For::= FOR LPAREN Def SEMI Expr SEMI Stat2 RPAREN Stat
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
		Val::= DECIMAL | INT | STRING
		");
	println!("0e");
	let grammar = grammar_generator(cfg);
	//println!("{:#?}", grammar);
	println!("1");
	let tokens = scanner(input, token_list);
	let ast = parser(tokens, grammar);
	println!("2");
	//let mut parse_tree = parser(tokens, grammar);
	//let ast = parse_tree_to_ast(&mut parse_tree, None);
	//println!("{:#?}", ast);
	let mut optimized_ast = optimize_ast(ast);
	//let opcodes = linearize_ast(optimized_ast, linearize as fn(&mut ASTNode, &mut Vec<Opcode>));
	let program = linearize_ast(&mut optimized_ast);
	println!("3");
	println!("{:#?}", program);
	//Make control flow graph?
	//let optimized_opcodes = optimize_opcodes(opcodes);

	return program;
}