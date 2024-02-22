use crate::lexer::{ Token, Name, Number };
use crate::error::Error;

use std::collections::HashMap;

pub type Tuple<'a> = Vec<Expr<'a>>;
pub type List<'a> = Vec<Expr<'a>>;
pub type Array<'a> = Vec<Expr<'a>>;
pub type Set<'a> = Vec<Expr<'a>>;
pub type Map<'a> = Vec<(Expr<'a>, Expr<'a>)>;

pub type ListType<'a> = Type<'a>;
pub type SetType<'a> = Type<'a>;
pub struct MapType<'a> { key: Type<'a>, value: Type<'a> }

pub struct Func<'a> { param: Name, expr: &'a Expr<'a> }

enum Literal<'a> {
	Number(Number),
	Char(char),
	String(String),
	Tuple(Tuple<'a>),
	List(List<'a>),
	Array(Array<'a>),
	Set(Set<'a>),
	Map(Map<'a>),
}
enum Type<'a> {
	NamedType(Name),
	FuncType(&'a Expr<'a>, &'a Expr<'a>),
	TupleType(Vec<&'a Expr<'a>>),
	ListType(&'a Expr<'a>),
	SetType(&'a Expr<'a>),
	MapType(&'a Expr<'a>, &'a Expr<'a>),
}
enum Pattern<'a> {
	NamePattern(Name),
	LiteralPattern(&'a Literal<'a>),
	EitherPattern(&'a Pattern<'a>, &'a Pattern<'a>),
	RangePattern(&'a Literal<'a>, &'a Literal<'a>),
	TuplePattern(Vec<Pattern<'a>>),
	ListPattern(Vec<Pattern<'a>>, Option<Name>),
	ArrayPattern(Vec<Pattern<'a>>, Option<Name>),
	SetPattern(Vec<Pattern<'a>>),
	MapPattern(Vec<(Pattern<'a>, Pattern<'a>)>),
}

pub enum Expr<'a> {
	Name(Name),
	Literal(Literal<'a>),
	Func(&'a Func<'a>),
	Application(&'a Expr<'a>, &'a Expr<'a>),
}

pub enum Block<'a> {
	LetDecl(Name, Type<'a>, Option<Expr<'a>>),
	LetDef(Name, Vec<Pattern<'a>>, Expr<'a>),
	ValDef(Name, Type<'a>, Expr<'a>),
	FuncDef(Name, Vec<Block<'a>>, Expr<'a>),
}

pub fn parse<'a>(tokens: Vec<Token>) -> Result<Vec<Block<'a>>, Error> {
	let procedures: Vec<Block>;
	Ok(procedures)
}
