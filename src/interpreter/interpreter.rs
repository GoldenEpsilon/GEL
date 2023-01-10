
use regex::Regex;
use std::collections::HashMap;
use std::vec;
use crate::interpreter::unwrap_values::*;
use crate::interpreter::operators::data_operation;
use crate::interpreter::builtin_variables::builtin_variables;
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
						}
					}//else if(){
						//object/self variable check
					//}
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
					//TODO: 
					// - Custom functions
					// - type checking
					// - input checking
					let mut data = vec![op.data.clone()];
					data.append(&mut unwrap_function_inputs(&op.data2, &registers, &variables));
					match &data[..] {
						[Data::Variable(func_name), arg] if func_name.to_string().as_str() == "print" || func_name.to_string().as_str() == "trace" => {
							println!("{}", get_value(arg, &registers, &variables).to_string());
							registers.insert(op.register, Data::Null);
						}
						[Data::Variable(func_name), Data::String(arg), Data::String(arg2)] if func_name.to_string().as_str() == "regex" => {
							registers.insert(op.register, Data::Int(Regex::new(&arg).unwrap().is_match(&arg2) as i32));
						}
						[Data::Variable(func_name), Data::Color(r,g,b,a)] if func_name.to_string().as_str() == "clear_background" => {
							clear_background(Color::new(r.to_f32().unwrap(), g.to_f32().unwrap(), b.to_f32().unwrap(), a.to_f32().unwrap()));
						}
						[Data::Variable(func_name), Data::String(str), Data::Decimal(x), Data::Decimal(y), Data::Decimal(fntsz), Data::Color(r,g,b,a)] if func_name.to_string().as_str() == "draw_text" => {
							draw_text(str.as_str(), x.to_f32().unwrap(), y.to_f32().unwrap(), fntsz.to_f32().unwrap(), Color::new(r.to_f32().unwrap(), g.to_f32().unwrap(), b.to_f32().unwrap(), a.to_f32().unwrap()));
						}
						[Data::Variable(func_name), Data::String(object_type)] if func_name.to_string().as_str() == "object_create" => {
							program.new_object(object_type.to_owned());
						}
						_ => {
							if let Data::Variable(func_name) = &data[0] { 
								if program.functions.contains_key(func_name) {
									registers.insert(op.register, interpret_program(program, func_name));
								}
							}
							/*if functions.contains_key(&data[0]) {
								func_stack.push(position);
								jump_point = functions[&data[0]].0;
							}else{
								match &data[0]{
									Data::Variable(func_name) => {
										println!("INVALID FUNCTION {} ON LINE {}", func_name, op.line);
									}
									dat => {
										println!("INVALID DATA TRYING TO BE FUNCTION: {:?}", dat);
									}
								}
							}*/
						}
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