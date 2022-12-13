
use std::collections::HashMap;
use crate::datatypes::Data;

pub fn get_value(data: &Data, registers: &HashMap<Data, Data>, variables: &HashMap<Data, (Data, Data)>) -> Data{
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

pub fn unwrap_function_inputs(data: &Data, registers: &HashMap<Data, Data>, variables: &HashMap<Data, (Data, Data)>) -> Vec<Data> {
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