/* Possible Token types */
#[derive(Debug)]
pub enum TK {
	Identifier,

	TInteger,
	TString,
	TFloat,
	TDouble,
	TProcedure,
	Integer,
	String,
	Float,
	Double,
	Procedure,

	Constant,
	Assign,
	AssignMut,
	BitwiseOr,
	LogicalOr,
	BitwiseAnd,
	LogicalAnd,
	NotEqual,
	Equal,
	LessThan,
	GreaterThan,
	LessEqualThan,
	GreaterEqualThan,
	OpenStatement,
	CloseStatement,

	IfStatement,
	WhileStatement,
	ForStatement,
	SwitchStatement,
}

#[derive(Debug)]
pub struct Tok {
	kind: TK,
	literal: String,
}


impl Tok {
	pub fn new(kind: TK, literal: String) -> Self {
		Self {
			kind: kind,
			literal: literal,
		}
	}
}