/* This code is pretty bad but works
 * Hopefully I can come back and fix it */

use crate::token::{Tok, TK};

pub struct Lexer {
	source: Vec<char>,
	current: usize,
	next: usize,
	char: char,
}

impl Lexer {
	pub fn new(source: String) -> Self {
		Self {
			source: source.chars().collect(),
			current: 0,
			next: 1,
			char: '\0',
		}
	}

	pub fn lex(&mut self) {
		let mut tokens: Vec<Tok> = Vec::new();

		while self.source.len() > self.current {
			let c = self.current_char();

			match c {
				'=' => {
					self.advance();
					if self.current_char() == '=' {
						tokens.push(Tok::new(TK::Equal, "==".to_owned()));
						self.advance();
					} else {
						tokens.push(Tok::new(TK::Assign, "=".to_owned()));
					}
				},
				
				':' => {
					self.advance();
					if self.current_char() == '=' {
						tokens.push(Tok::new(TK::AssignMut, ":=".to_owned()));
					}

					if self.current_char() == ':' {
						tokens.push(Tok::new(TK::Constant, "::".to_owned()));
					}

					self.advance();
				},

				'|' => {
					self.advance();
					if self.current_char() == '|' {
						tokens.push(Tok::new(TK::LogicalOr, "||".to_owned()));
						self.advance();
					} else {
						tokens.push(Tok::new(TK::BitwiseOr, "|".to_owned()));
					}
				},

				'&' => {
					self.advance();
					if self.current_char() == '&' {
						tokens.push(Tok::new(TK::LogicalAnd, "&&".to_owned()));
						self.advance();
					} else {
						tokens.push(Tok::new(TK::BitwiseAnd, "&".to_owned()))
					}
				},

				'<' => {
					self.advance();
					if self.current_char() == '=' {
						tokens.push(Tok::new(TK::LessEqualThan, "<=".to_owned()));
						self.advance();
					} else {
						tokens.push(Tok::new(TK::LessThan, "<".to_owned()));
					}
				},

				'>' => {
					self.advance();
					if self.current_char() == '=' {
						tokens.push(Tok::new(TK::GreaterEqualThan, ">=".to_owned()));
						self.advance();
					} else {
						tokens.push(Tok::new(TK::GreaterThan, ">".to_owned()));
					}
				},

				'\"' => {
					let mut buf = String::new();
					self.advance();

					while self.current_char() != '\"' {
						buf.push(self.current_char());
						self.advance();
					}

					tokens.push(Tok::new(TK::String, buf));
					self.advance();
				},

				'{' => {
					tokens.push(Tok::new(TK::OpenStatement, "{".to_owned()));
					self.advance();
				},

				'}' => {
					tokens.push(Tok::new(TK::CloseStatement, "}".to_owned()));
				}


				_ if c.is_alphabetic() => {
					let mut buf = String::new();
					buf.push(c);
					
					self.advance();

					while self.current_char().is_alphabetic() {
						buf.push(self.current_char());
						self.advance();
					}

					let kind = match &buf[..] {
						"proc" => TK::Procedure,
						"if" =>  TK::IfStatement,
						"for" => TK::ForStatement,
						"while" => TK::WhileStatement,
						"switch" => TK::SwitchStatement,

						"int" => TK::TInteger,
						"string" => TK::TString,
						"float" => TK::TFloat,
						"double" => TK::TDouble,

						_ => TK::Identifier,
					};

					tokens.push(Tok::new(kind, buf));
				},


				_ if c.is_numeric() => {
					let mut buf = String::new();
					let mut is_float = false;
					buf.push(c);
					self.advance();

					loop {
						if self.current >= self.source.len() {
							break;
						}

						if self.current_char() == '.' {
							is_float = true;
						}

						buf.push(self.current_char());
						self.advance();
					}

					if is_float {
						tokens.push(Tok::new(TK::Float, buf));
					} else {
						tokens.push(Tok::new(TK::Integer, buf));
					}
				},

				_ => (),
			}

			self.advance();	
		}

		println!("{:?}", tokens);

	}

	/* Misc */
	pub fn current_char(&self) -> char {
		*self.source.get(self.current).unwrap()
	}

	pub fn advance(&mut self) {
		self.current += 1;
		self.next += 1;
	}
}