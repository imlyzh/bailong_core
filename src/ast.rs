use std::{collections::HashMap, ops::BitAnd, sync::Arc};

use rustpython_parser::ast;

use crate::type_system::*;
use crate::utils::string_intern;

#[derive(Debug, Clone)]
pub struct Location {
    row: usize,
    column: usize,
}

impl Location {
	pub fn from(l: ast::Location) -> Self {
		Self {
			row: l.row(),
			column: l.column()
		}
	}
}


pub type Handle<T> = Arc<T>;

#[derive(Debug, Clone)]
pub struct Symbol (Handle<String>);

impl Symbol {
	pub fn new(s: &str) -> Self {
		Self (string_intern(s))
	}
}

unsafe impl Sync for Symbol {}
unsafe impl Send for Symbol {}


#[derive(Debug, Clone)]
pub enum AST {
	Function(Handle<Function>),
	Record(Handle<Record>),
	// Bind(Handle<Bind>)
}

#[derive(Debug, Clone)]
pub struct Record (pub HashMap<Symbol, Type>, pub HashMap<Symbol, Function>);

#[derive(Debug, Clone)]
pub struct Bind (Expr, Expr);

#[derive(Debug, Clone)]
pub struct Function {
	pub name: Symbol,
	pub location: Location,
	pub funtype: Vec<Type>,
	pub rettype: Type,
	pub args: Vec<Symbol>,
	pub body: Expr
}

#[derive(Debug, Clone)]
pub enum Expr {

}