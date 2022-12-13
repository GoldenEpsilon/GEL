use crate::datatypes::Data;

pub fn data_operation(left: Data, right: Data, op: String) -> Data {
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