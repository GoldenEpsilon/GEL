
use regex::Regex;
use std::collections::HashMap;
use std::vec;
use crate::interpreter::unwrap_values::*;
use crate::interpreter::operators::data_operation;
use crate::interpreter::builtin_variables::builtin_variables;
use crate::interpreter::builtin_functions::run_builtin;
use crate::datatypes::*;
use macroquad::prelude::*;
use rust_decimal::prelude::*;

pub fn interpret_program(program: &mut Program, startingfunction: &str) -> Data{
	let opcodes;
	{
		let function = program.functions.get(startingfunction);
		if function.is_none() {
			return Data::Null;
		}
		opcodes = function.unwrap().1.to_owned();
	}
	let mut registers: HashMap<u32, Data> = HashMap::new();
	//let mut labels: HashMap<Data, u32> = HashMap::new();
	let mut variables: HashMap<String, (Data, Data)> = builtin_variables();
	//let mut functions: HashMap<Data, (u32, Vec<(Data, Data)>)> = HashMap::new();
	let mut jump_point;
	let mut func_stack: Vec<u32> = vec![];
	let mut _position = 1;
	/*for op in opcodes {
		match op.instruction.as_str() {
			"FOR_LABEL" | "IF_LABEL" | "ELSE_LABEL" => {
				labels.insert(op.data.clone(), position);
			}
			/*"FUNC_DEF" => {
				functions.insert(op.data.clone(), (position, unwrap_function_definition(position, &opcodes)));
			}*/
			_ => {}
		}
		position += 1;
	}*/
	/*let startfn = Data::Variable(startingfunction.to_string());
	if functions.contains_key(&startfn) {
		jump_point = functions[&startfn].0;
	}else if startingfunction == "init" {
		jump_point = 1;
	} else {
		//println!("{} is not a valid function", startingfunction);
		return;
	}*/
	jump_point = 1;
	while jump_point != 0 {
		let iter = opcodes.iter().skip(jump_point.to_usize().unwrap()-1);
		_position = jump_point;
		jump_point = 0;
		for op in iter {
			//println!("{:?}", op);
			//println!("{:?}", position);
			match op.instruction.as_str() {
				"NOP" => {}
				"Declare" => {
					if let Data::Variable(data2) = &op.data2 {
						variables.insert(data2.to_owned(), (op.data.clone(), Data::Null));
					}
				}
				"Set" => {
					if let Data::Variable(data) = &op.data {
						if variables.contains_key(data) {
							if let Data::Register(data2) = op.data2 {
								if registers.contains_key(&data2) {
									variables.get_mut(&data.to_owned()).unwrap().1 = registers.get_mut(&data2).unwrap().clone();
								}
							}
						}//else if(program.){
							//object/self variable check
						//}
					}
				}
				"Value" => {
					registers.insert(op.register, op.data.clone());
				}
				"ID" => {
					registers.insert(op.register, op.data.clone());
				}
				"Comma" => {
					registers.insert(op.register, Data::Comma(Box::new(op.data.clone()), Box::new(op.data2.clone())));
				}
				"FUNC" => {
					if let Data::Variable(func) = get_value(&op.data, &registers, &variables){
						let data = run_builtin(func.as_str(), unwrap_function_inputs(&op.data2, &registers, &variables), &registers, &variables, program);
						if data.is_none() {
							if program.functions.contains_key(&func) {
								registers.insert(op.register, interpret_program(program, &func));
							}else{
								panic!("ERROR: FUNCTION {} DOES NOT EXIST ON LINE {}", func, op.line);
							}
						}else{
							registers.insert(op.register, data.unwrap());
						}
					} else {
						panic!("ERROR: {} IS NOT A DATATYPE THAT CAN BE A FUNCTION ON LINE {}", op.data, op.line);
					}
				}
				"ARG" => {}
				"FUNC_ARGS" => {}
				"FUNC_DEF" => {}
				"END_FUNC" => {
					if func_stack.len() > 0 {
						func_stack.pop();
					}else{
						jump_point = 0;
						break;
					}
				}
				"PLUS" | "MINUS" | "MULT" | "DIV" | "EXP" | "GT" | "LT" | "EQ" | "AND" | "OR" | "DOT" => {
					let left = get_value(&op.data, &registers, &variables);
					let right = get_value(&op.data2, &registers, &variables);
					registers.insert(op.register, data_operation(left, right, op.instruction.clone()));
					//println!("{:?}", registers.get(&Data::Register(op.register)));
				}
				"INCR" => {
					if let Data::Variable(data) = &op.data {
						variables.get_mut(data).unwrap().1 = data_operation(get_value(&op.data, &registers, &variables), Data::Null, op.instruction.clone());
					}
				}
				"IF_GOTO" => {
					if let Data::Label(label) = op.data2 {
						jump_point = program.labels[label];
					}
					//println!("if: {}, {}", position, jump_point);
				}
				"FOR_GOTO" => {
					match get_value(&op.data, &registers, &variables) {
						Data::Null => {}
						Data::Int(i) if i == 0 => {}
						_ => {
							if let Data::Label(label) = op.data2 {
								jump_point = program.labels[label];
							}
						}
					}
				}
				"ELSE_GOTO" => {
					match get_value(&op.data, &registers, &variables) {
						Data::Null => {
							if let Data::Label(label) = op.data2 {
								jump_point = program.labels[label];
							}
						}
						Data::Int(i) if i == 0 => {
							if let Data::Label(label) = op.data2 {
								jump_point = program.labels[label];
							}
						}
						_ => {}
					}
					//println!("else: {}, {}", position, jump_point);
				}
				opcode => {
					println!("UNKNOWN OPCODE ON LINE {}. OPCODE IS {}", op.line, opcode);
				}
			}
			if jump_point != 0 {
				break;
			}
			_position += 1;
		}
	}
	return Data::Null;
	//println!("{}{:#?}", "Registers: ", registers);
	//println!("{}{:#?}", "Variables: ", variables);
}