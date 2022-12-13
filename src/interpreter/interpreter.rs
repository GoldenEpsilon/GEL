
use regex::Regex;
use std::collections::HashMap;
use crate::interpreter::unwrap_values::unwrap_function_inputs;
use crate::interpreter::unwrap_values::get_value;
use crate::interpreter::operators::data_operation;
use crate::datatypes::Data;
use crate::datatypes::Opcode;

pub fn interpret_opcodes(opcodes: Vec<Opcode>){
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