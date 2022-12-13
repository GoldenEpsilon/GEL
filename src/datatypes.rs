use std::fmt;

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
	pub lookahead: Vec<String>
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
	pub register: i32,
	pub line: i32,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
pub enum Data {
    Null,
    Int(i32),
    String(String),
    Register(i32),
    Label(i32),
    Variable(String),
    Type(String),
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
			Data::Int(data) => {
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
		}
    }
}