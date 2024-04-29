
use std::collections::HashMap;
use crate::datatypes::Data;
//use crate::datatypes::Opcode;

pub fn get_value(data: &Data, registers: &HashMap<u32, Data>, variables: &HashMap<String, (Data, Data)>) -> Result<Data, String>{
	match data {
		Data::Register(true_data) => {
			let reg = registers.get(&true_data);
			if !reg.is_some() {
				return Err(format!("NONEXISTENT REGISTER ACCESS TRYING TO ACCESS {:?}", data));
			}
			return get_value(&reg.unwrap(), registers, variables);
		}
		Data::Variable(true_data) => {
			let var = variables.get(true_data);
			if !var.is_some() {
				//probably a function, regardless just return null to make it happy
				//return Data::Null;
				return Err(format!("NONEXISTENT VARIABLE ACCESS TRYING TO ACCESS {:?}", data));
			}
			return get_value(&var.unwrap().1, registers, variables);
		}
		_ => {
			return Ok(data.clone());
		}
	};
}

pub fn unwrap_function_inputs(data: &Data, registers: &HashMap<u32, Data>, variables: &HashMap<String, (Data, Data)>) -> Result<Vec<Data>, String> {
	let mut ret_val = vec![];
	match data {
		Data::Comma(l, r) => {
			ret_val.append(&mut unwrap_function_inputs(l, registers, variables)?);
			ret_val.append(&mut unwrap_function_inputs(r, registers, variables)?);
		}
		Data::Register(_) | Data::Variable(_) => {
			ret_val.append(&mut unwrap_function_inputs(&get_value(data, registers, variables)?, registers, variables)?);
		}
		Data::Null => {
		}
		val => {
			ret_val.push(val.clone());
		}
	}
	return Ok(ret_val);
}

/*
pub fn unwrap_function_definition(position: usize, opcodes: &Vec<Opcode>) -> Vec<(Data, Data)> {
	let mut ret_val = vec![];
	let mut counter = 1;
	for op in &opcodes[position..] {
		match op.instruction.as_str() {
			"FUNC_ARGS" => {
				match op.data {
					Data::Int(len) => {
						counter += len;
					}
					_ => {}
				}
			}
			"ARG" => {
				ret_val.push((op.data2.to_owned(), op.data.to_owned()));
			}
			_ => {}
		}
		counter -= 1;
		if counter <= 0 {
			break;
		}
	}
	return ret_val;
}*/