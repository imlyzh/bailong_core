use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;
use rustpython_parser::ast as pyast;

use crate::{ast::Handle, ast::Location, error};

lazy_static! {
    static ref GLOBAL_INTERN_STRING_POOL: Mutex<HashMap<Handle<String>, Handle<String>>> =
	Mutex::new(HashMap::new());
}

#[inline]
pub fn string_intern(i: &str) -> Handle<String> {
    let k = Handle::new(i.to_string());
    {
        if let Some(x) = GLOBAL_INTERN_STRING_POOL.lock().unwrap().get(&k) {
            return x.clone();
        }
    }
    GLOBAL_INTERN_STRING_POOL
        .lock()
        .unwrap()
        .insert(k.clone(), k.clone());
    k
}

pub fn invalid_code_error<T>(location: pyast::Location) -> Result<T, error::Error> {
	Err(error::Error::InvalidCode(Some(Location::from(location))))
}