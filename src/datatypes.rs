use std::fmt;
use std::collections::HashMap;
use derivative::Derivative;
use futures::executor;
use macroquad::prelude::load_texture;
use macroquad::texture::Texture2D;
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
    Object(usize),
    Comma(Box<Data>, Box<Data>),
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
			Data::Decimal(data) => {
				write!(f, "{}", data)
			}
			Data::Int(data) => {
				write!(f, "{}", data)
			}
			Data::String(data) => {
				write!(f, "{}", data)
			}
			Data::Color(data1, data2, data3, data4) => {
				write!(f, "Color({}, {}, {}, {})", data1, data2, data3, data4)
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
			Data::Object(data) => {
				write!(f, "Object {}", data)
			}
			Data::Comma(data1, data2) => {
				write!(f, "{}, {}", data1.to_string(), data2.to_string())
			}
		}
    }
}

#[derive(Debug)]
#[derive(Clone)]
//Implement partialeq and eq by using id
#[derive(PartialEq)]
#[derive(Eq)]
pub struct Object {
	pub id: usize,
	pub name: String,
	pub object_type: String,
	pub data: HashMap<String, Data>
}
impl std::hash::Hash for Object {
	fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        state.write_usize(self.id);
        state.finish();
    }
}
impl Object {
	pub fn new(object_type: String, id: usize) -> Object {
		return Object{id, name: object_type.to_owned(), object_type, data: HashMap::new()};
	}
}

#[derive(Derivative)]
#[derivative(PartialEq, Eq, Hash, Clone, Debug)]
pub struct SpriteData {
	pub path: String,
	pub name: String,
	#[derivative(PartialEq="ignore")]
    #[derivative(Hash="ignore")]
	pub texture: Texture2D,
}
impl SpriteData {
	pub fn new(path: String, name: String, program: &mut Program) -> String {
		let tex = executor::block_on(load_texture(path.as_str()));
		let ret_val;
		match tex {
			Ok(texture) => {
				ret_val = SpriteData{texture: texture, path: path, name: name.to_owned()};
			}
			Err(e) => {
				println!("ERROR LOADING TEXTURE {}: {:?}", path, e);
				ret_val = SpriteData{texture: Texture2D::empty(), path: path, name: name.to_owned()};
			}
		}
		program.sprites.insert(name.to_owned(), ret_val);
		return name;
	}
}

#[derive(Debug)]
pub struct FuncData {
	pub return_type: Data,
	pub input_types: Vec<Data>,
	pub optional_types: HashMap<String, Data>,
}

#[derive(Debug)]
#[derive(Default)]
pub struct Program {
	pub initialized: bool,
	pub current_frame: i32,
	pub functions: HashMap<String, (FuncData, Vec<Opcode>)>,
	pub labels: Vec<usize>,
	pub sprites: HashMap<String, SpriteData>,
	objects: Vec<Object>,
	objects_sorted: HashMap<String, Vec<usize>>,
	id_index: usize,
	context: Vec<usize>,
	pub log: Vec<String>
}

#[derive(Debug)]
pub struct Console {
	pub open: bool,
	pub just_opened: bool,
	pub console_text: String,
	pub console_log: Vec<(String, u32)>,
	pub console_history: Vec<String>,
	pub index: usize
}

impl Program {
	pub fn new() -> Program {
		return Program{
			functions: HashMap::new(), 
			labels: vec![], 
			sprites: HashMap::new(),
			objects: vec![Object::new("Program".to_string(), 1)], 
			objects_sorted: HashMap::from([("Program".to_string(), vec![1])]), 
			id_index: 1, 
			context: vec![1],
			log: vec![],
			..Default::default()
		};
	}
	pub fn new_object(&mut self, object_type: String) -> usize {
		self.id_index += 1;
		let obj = Object::new(object_type.to_owned(), self.id_index);
		self.objects.push(obj);
		self.objects_sorted.insert(object_type, vec![self.id_index]);
		return self.id_index;
	}
	pub fn get_object(&mut self, id: usize) -> &mut Object {
		return &mut self.objects[id];
	}
	pub fn enter_context(&mut self, object: usize) {
		self.context.push(object);
	}
	pub fn exit_context(&mut self) {
		self.context.pop();
	}
	pub fn get_self(&mut self) -> usize {
		let length = self.context.len();
		if length > 0 {
			return self.context[length - 1];
		}
		return 0;
	}
	pub fn get_other(&mut self) -> usize {
		let length = self.context.len();
		if length > 1 {
			return self.context[length - 2];
		}
		return 0;
	}
}