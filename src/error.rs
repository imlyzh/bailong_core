use crate::ast::Location;

pub enum Error {
	InvalidCode(Option<Location>),
}