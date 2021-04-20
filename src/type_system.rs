use std::cell::RefCell;
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::ast::Symbol;


#[derive(Debug, Clone)]
pub enum Type {
	None,
	U8(u8, u8),
	U16(u16, u16),
	U32(u32, u32),
	U64(u64, u64),
	I8(i8, i8),
	I16(i16, i16),
	I32(i32, i32),
	I64(i64, i64),
	F32(f32, f32),
	F64(f64, f64),
	String(Vec<String>, Option<usize>),
	Bytes(usize),
	Union(Vec<Type>),
	Name(Symbol),
	TApply(Box<Type>, Box<Type>),
	TVar(RefCell<TypeVar>),
}


impl Type {
	pub fn new() -> Type {
		Type::TVar(
			RefCell::new(TypeVar::TypeId(new_id()))
		)
	}
}


lazy_static! {
    static ref TYPE_COUNT: Mutex<usize> =
	Mutex::new(0);
}

fn new_id() -> usize {
	let mut h= TYPE_COUNT.lock().unwrap();
	let r = *h;
	*h+=1;
	r
}

fn reset_id() -> () {
	let mut h = TYPE_COUNT.lock().unwrap();
	*h=0;
}


#[derive(Debug, Clone)]
pub enum TypeVar {
	TypeId(usize),
	TypeName(Symbol, usize),
}
