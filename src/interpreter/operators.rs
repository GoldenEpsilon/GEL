use rust_decimal::{Decimal, prelude::{ToPrimitive, FromPrimitive}, MathematicalOps};
use rust_decimal_macros::dec;
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

		(Data::Decimal(l), Data::Decimal(r), "PLUS") => {
			return Data::Decimal(l + r);
		}
		(Data::Decimal(l), Data::Decimal(r), "MINUS") => {
			return Data::Decimal(l - r);
		}
		(Data::Decimal(l), Data::Decimal(r), "MULT") => {
			return Data::Decimal(l * r);
		}
		(Data::Decimal(l), Data::Decimal(r), "DIV") => {
			return Data::Decimal(l / r);
		}
		(Data::Decimal(l), Data::Decimal(r), "EXP") => {
			return Data::Decimal(l.powf(r.to_f64().unwrap()));
		}
		(Data::Decimal(l), Data::Decimal(r), "GT") => {
			return Data::Decimal(Decimal::from_i8((l > r) as i8).unwrap());
		}
		(Data::Decimal(l), Data::Decimal(r), "LT") => {
			return Data::Decimal(Decimal::from_i8((l < r) as i8).unwrap());
		}
		(Data::Decimal(l), Data::Decimal(r), "EQ") => {
			return Data::Decimal(Decimal::from_i8((l == r) as i8).unwrap());
		}
		(Data::Decimal(l), Data::Decimal(r), "AND") => {
			return Data::Decimal(Decimal::from_i8((l != dec!(0) && r != dec!(0)) as i8).unwrap());
		}
		(Data::Decimal(l), Data::Decimal(r), "OR") => {
			return Data::Decimal(Decimal::from_i8((l != dec!(0) || r != dec!(0)) as i8).unwrap());
		}
		(Data::Decimal(l), Data::Null, "INCR") => {
			return Data::Decimal(l + dec!(1));
		}
		_ => {
			return Data::Null;
		}
	}
}