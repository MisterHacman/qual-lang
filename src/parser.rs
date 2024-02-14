use crate::lexer::{ Token, Name, Number };
use crate::error::Error;

use std::collections::HashMap;

pub type Tuple<'a> = Vec<Expr<'a>>;
pub type List<'a> = Vec<Expr<'a>>;
pub type Array<'a> = Vec<Expr<'a>>;
pub type Set<'a> = Vec<Expr<'a>>;
pub type Map<'a> = Vec<(Expr<'a>, Expr<'a>)>;

pub type Literal<'a> = Expr<'a>;

pub type Type<'a> = Expr<'a>;
pub struct Func<'a> { param: Name, expr: &'a Expr<'a> }
pub type Pattern<'a> = Expr<'a>;

pub enum Expr<'a> {
	Name(Name),

	Number(Number),
	Char(char),
	String(String),
	Tuple(Tuple<'a>),
	List(List<'a>),
	Array(Array<'a>),
	Set(Set<'a>),
	Map(Map<'a>),

	Func(&'a Func<'a>),
	Application(&'a Expr<'a>, Vec<&'a Expr<'a>>),

	NamedType(Name),
	FuncType(&'a Expr<'a>, &'a Expr<'a>),
	TupleType(Vec<&'a Expr<'a>>),
	ListType(&'a Expr<'a>),
	SetType(&'a Expr<'a>),
	MapType(&'a Expr<'a>, &'a Expr<'a>),

	LiteralPattern(&'a Literal<'a>),
	EitherPattern(&'a Pattern<'a>, &'a Pattern<'a>),
	RangePattern(&'a Literal<'a>, &'a Literal<'a>),
	TuplePattern(Vec<Pattern<'a>>),
	ListPattern(Vec<Pattern<'a>>, Option<Name>),
	ArrayPattern(Vec<Pattern<'a>>, Option<Name>),
	SetPattern(Vec<Pattern<'a>>),
	MapPattern(Vec<(Pattern<'a>, Pattern<'a>)>),
}

pub enum Block<'a> {
	LetDeclaration(Name, Expr<'a>, Option<(Option<Vec<Name>>, Expr<'a>)>),
	LetDefinition(Name, Vec<Pattern<'a>>, Expr<'a>),
	ValDefinition(Name, Expr<'a>, Expr<'a>),
	FunctionDefinition(Name, Vec<Block<'a>>, Expr<'a>),
}

pub fn parse<'a>(tokens: Vec<Token>) -> Result<HashMap<String, Vec<Block<'a>>>, Error> {
	let procedures: HashMap<String, Vec<Block>>;
	Ok(procedures)
}
