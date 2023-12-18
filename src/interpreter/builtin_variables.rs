use std::collections::HashMap;
use crate::datatypes::Data;
use rust_decimal::prelude::*;

pub fn builtin_variables() -> HashMap<String, (Data, Data)> {
    let mut ret_val = HashMap::new();
    new_builtin_color(&mut ret_val, "c_dgray", 0.31, 0.31, 0.31, 1.0);
    new_builtin_color(&mut ret_val, "c_red", 1.0, 0.0, 0.0, 1.0);
    new_builtin_color(&mut ret_val, "c_white", 1.0, 1.0, 1.0, 1.0);
    new_builtin_generic(&mut ret_val, "test", "int", Data::Int(1));
    return ret_val;
}

fn new_builtin_generic(builtins: &mut HashMap<String, (Data, Data)>, name: &str, typ: &str, val: Data){
    builtins.insert(name.to_string(), (Data::Type(typ.to_string()), val));
}

fn new_builtin_color(builtins: &mut HashMap<String, (Data, Data)>, name: &str, r: f32, g: f32, b: f32, a: f32){
    builtins.insert(name.to_string(), (Data::Type("Color".to_string()), Data::Color(Decimal::from_f32(r).unwrap(), Decimal::from_f32(g).unwrap(), Decimal::from_f32(b).unwrap(), Decimal::from_f32(a).unwrap())));
}