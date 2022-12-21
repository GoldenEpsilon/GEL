use crate::datatypes::ASTNode;
use crate::datatypes::Opcode;
use crate::datatypes::Data;
use rust_decimal::prelude::*;

pub fn linearize(ast: &mut ASTNode, curr_reg: &mut i32) -> Vec<Opcode> {
	let mut ret_val: Vec<Opcode> = vec![];
	match ast.rule.as_str(){
		"Root" => {
			for i in &mut ast.children {
				ret_val.append(&mut linearize(i, curr_reg));
			}
		}
		"For" => {
			ret_val.append(&mut linearize(&mut ast.children[2], curr_reg));
		}
		"For2" => {
			let paren;
			if ast.children.len() == 8 {
				paren = 1;
			}else{
				paren = 0;
			}
			
			ret_val.append(&mut linearize(&mut ast.children[1 - paren], curr_reg));
			
			//label to goto
			ret_val.push(Opcode{instruction: "FOR_LABEL".to_string(), data: Data::Label(*curr_reg), data2: Data::Null, register: 0, line: 0});
			let label_reg = *curr_reg;
			*curr_reg += 1;
			
			//Block
			ret_val.append(&mut linearize(&mut ast.children[7 - paren * 2], curr_reg));
			
			//Loop Check
			ret_val.append(&mut linearize(&mut ast.children[3 - paren], curr_reg));
			let reg = Data::Register(ret_val.last().unwrap().register);
			
			//Modify iterator
			ret_val.append(&mut linearize(&mut ast.children[5 - paren], curr_reg));
			
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
			if ast.children.len() == 3 {
				ret_val.push(Opcode{instruction: "FUNC_DEF".to_string(), data: Data::Variable(ast.children[1].data.as_ref().unwrap().1.to_owned()), data2: Data::Null, register: *curr_reg, line: 0});
				*curr_reg += 1;
				ret_val.append(&mut linearize(&mut ast.children[2], curr_reg));
				ret_val.push(Opcode{instruction: "END_FUNC".to_string(), data: Data::Variable(ast.children[1].data.as_ref().unwrap().1.to_owned()), data2: Data::Null, register: *curr_reg, line: 0});
				*curr_reg += 1;
			} else {
				ret_val.append(&mut linearize(&mut ast.children[3], curr_reg));
				ret_val.push(Opcode{instruction: "FUNC_DEF".to_string(), data: Data::Variable(ast.children[2].data.as_ref().unwrap().1.to_owned()), data2: Data::Register(ret_val[ret_val.len() - 1].register), register: *curr_reg, line: 0});
				*curr_reg += 1;
				ret_val.append(&mut linearize(&mut ast.children[5], curr_reg));
				ret_val.push(Opcode{instruction: "END_FUNC".to_string(), data: Data::Variable(ast.children[2].data.as_ref().unwrap().1.to_owned()), data2: Data::Null, register: *curr_reg, line: 0});
				*curr_reg += 1;
			}
		}
		"FuncDef2" => {
			if ast.children.len() == 4 {
				let mut child = linearize(&mut ast.children[1], curr_reg);
				ret_val.push(Opcode{instruction: "FUNC_ARGS".to_string(), data: Data::Int(child.len() as i32), data2: Data::Null, register: *curr_reg, line: 0});
				*curr_reg += 1;
				ret_val.append(&mut child);
				ret_val.append(&mut linearize(&mut ast.children[3], curr_reg));
			} else {
				ret_val.push(Opcode{instruction: "FUNC_ARGS".to_string(), data: Data::Null, data2: Data::Null, register: *curr_reg, line: 0});
				*curr_reg += 1;
				ret_val.append(&mut linearize(&mut ast.children[0], curr_reg));
			}
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
		"Arg" => {
			if ast.children.len() == 3 {
				ret_val.push(Opcode{instruction: "ARG".to_string(), data: Data::Variable(ast.children[1].data.as_ref().unwrap().1.to_owned()), data2: Data::Type(ast.children[0].data.as_ref().unwrap().1.to_owned()), register: 0, line: 0});
				ret_val.append(&mut linearize(&mut ast.children[2], curr_reg));
			}else{
				ret_val.push(Opcode{instruction: "ARG".to_string(), data: Data::Variable(ast.children[0].data.as_ref().unwrap().1.to_owned()), data2: Data::Type("var".to_string()), register: 0, line: 0});
				ret_val.append(&mut linearize(&mut ast.children[1], curr_reg));
			}
		}
		"ArgDefault" => {
			if ast.children.len() == 2 {
				ret_val.append(&mut linearize(&mut ast.children[1], curr_reg));
			}
		}
		"ID" => {
			ret_val.push(Opcode{instruction: "ID".to_string(), data: Data::Variable(ast.data.as_ref().unwrap().1.to_owned()), data2: Data::Null, register: *curr_reg, line: 0});
			*curr_reg += 1;
		}
		"FLOAT" => {
			if !Decimal::from_str(ast.data.as_ref().unwrap().1.as_str()).is_err() {
				ret_val.push(Opcode{instruction: "Value".to_string(), data: Data::Decimal(Decimal::from_str(ast.data.as_ref().unwrap().1.as_str()).unwrap()), data2: Data::Null, register: *curr_reg, line: 0});
				*curr_reg += 1;
			}
		}
		"INT" => {
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
		"Def" => {
			let mut child = linearize(&mut ast.children[1], curr_reg);
			if child.len() > 0 && child[0].instruction == "Declare".to_string() {
				child[0].data = Data::Type(ast.children[0].data.as_ref().unwrap().1.to_owned());
				ret_val.append(&mut child);
			}
		}
		"Def2" => {
			let mut child = linearize(&mut ast.children[0], curr_reg);
			ret_val.append(&mut child);
		}
		"Decl" => {
			let mut child = linearize(&mut ast.children[1], curr_reg);
			ret_val.push(Opcode{instruction: "Declare".to_string(), data: Data::Null, data2: Data::Variable(ast.children[0].data.as_ref().unwrap().1.to_owned()), register: 0, line: ast.line});
			if child.len() > 0{
				let reg = Data::Register(child[child.len() - 1].register);
				ret_val.append(&mut child);
				ret_val.push(Opcode{instruction: "Set".to_string(), data: Data::Variable(ast.children[0].data.as_ref().unwrap().1.to_owned()), data2: reg, register: *curr_reg, line: ast.line});
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