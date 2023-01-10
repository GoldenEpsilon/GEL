use std::fmt;
use std::collections::HashMap;
use rust_decimal::prelude::*;

#[derive(Clone, Copy)]
pub enum TokenAction {
	Keywords,
	Identity,
	Bool,
	HexNum,
	Newline,
	Comment,
	Whitespace,
}

#[derive(Clone)]
#[derive(Debug)]
pub struct GrammarToken {
	pub is_terminal: bool,
	pub value: String,
	pub lookahead: Vec<String>,
	pub is_subrule: bool
}

#[derive(Debug)]
#[derive(Clone)]
pub struct ASTNode {
	pub rule: String,
	pub data: Option<(String, String, i32)>,
	pub children: Vec<ASTNode>,
	pub line: i32,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Opcode {
	pub instruction: String,//might want to replace this with something more useful and/or faster, like a direct function reference
	pub data: Data,
	pub data2: Data,
	pub register: u32,
	pub line: i32,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
pub enum Data {
    Null,
    Decimal(Decimal),
    Int(i32),
    String(String),
    Color(Decimal, Decimal, Decimal, Decimal),
    Register(u32),
    Label(usize),
    Variable(String),
    Type(String),
    Comma(Box<Data>, Box<Data>),
    //Object(), //make it a reference to the list of objects
}
impl fmt::Display for Data {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
		match self {
			Data::Null => {
				write!(f, "NULL")
			}
			Data::Int(data) => {
				write!(f, "{}", data)
			}
			Data::Color(data1, data2, data3, data4) => {
				write!(f, "Color({}, {}, {}, {})", data1, data2, data3, data4)
			}
			Data::Decimal(data) => {
				write!(f, "{}", data)
			}
			Data::String(data) => {
				write!(f, "{}", data)
			}
			Data::Register(data) => {
				write!(f, "Register {}", data)
			}
			Data::Label(data) => {
				write!(f, "Label {}", data)
			}
			Data::Variable(data) => {
				write!(f, "variable {}", data)
			}
			Data::Type(data) => {
				write!(f, "type {}", data)
			}
			Data::Comma(data1, data2) => {
				write!(f, "{}, {}", data1.to_string(), data2.to_string())
			}
			/*Data::Object(data) => {
				write!(f, "{:?}", data.name)
			}*/
		}
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct Object {
	pub id: u32,
	pub name: String,
	pub object_type: String,
	pub data: HashMap<String, Data>
}
impl std::hash::Hash for Object {
	fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        state.write_u32(self.id);
        state.finish();
    }
}
impl Object {
	pub fn new(object_type: String, id: u32) -> Object {
		return Object{id, name: object_type.to_owned(), object_type, data: HashMap::new()};
	}
}

#[derive(Debug)]
pub struct FuncData {
	pub return_type: Data,
	pub input_types: Vec<Data>,
	pub optional_types: HashMap<String, Data>,
}

#[derive(Debug)]
pub struct Program {
	pub functions: HashMap<String, (FuncData, Vec<Opcode>)>,
	pub labels: Vec<usize>,
	pub objects: HashMap<String, Vec<Object>>,
	pub id_index: u32
}

impl Program {
	pub fn new() -> Program {
		return Program{functions: HashMap::new(), labels: vec![], objects: HashMap::from([("Program".to_string(), vec![Object::new("Program".to_string(), 1)])]), id_index: 1};
	}
	pub fn new_object(&mut self, object_type: String) -> Object {
		self.id_index += 1;
		return Object::new(object_type, self.id_index);
	}
}