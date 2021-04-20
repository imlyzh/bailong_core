use rustpython_parser::ast as pyast;

use crate::{error::Error};
use crate::type_system::*;
use crate::{ast::*, utils::invalid_code_error};

fn stmt_transform(code: &pyast::Statement) -> Result<Expr, Error> {
    let node = &code.node;
    match node {
        pyast::StatementType::Expression { expression } => {
            todo!()
        }
        _ => invalid_code_error(code.location),
    }
}

fn get_type(typ: &pyast::Expression) -> Result<Type, Error> {
    match typ.node {
        pyast::ExpressionType::Binop {
            ref op,
            ref a,
            ref b,
        } if *op == pyast::Operator::BitOr => {
			let (a, b) =(
				get_type(a)?,
				get_type(b)?);
			let r = match (a.clone(), b.clone()) {
			    (Type::Union(mut a), Type::Union(mut b)) => {
					a.append(&mut b);
					Type::Union(a)
				}
				(Type::Union(mut a), b) |
				(b, Type::Union(mut a)) => {
					a.push(b);
					Type::Union(a)
				}
				_ => Type::Union(vec![a, b])
			};
			Ok(r)
        }
        pyast::ExpressionType::String { ref value } => {
			if let pyast::StringGroup::Constant{value} = value {
				Ok(Type::String(vec![value.clone()], None))
			} else {
				Err(Error::InvalidCode(Some(Location::from(typ.location))))
			}
        }
		pyast::ExpressionType::Subscript {
			ref a,
			ref b } => {
            let a = get_type(a)?;
			let b = get_type(b)?;
			Ok(Type::TApply(Box::new(a), Box::new(b)))
        }
        pyast::ExpressionType::Identifier { ref name } => {
            Ok(Type::Name(Symbol::new(&name)))
        }
        pyast::ExpressionType::None => {
            Ok(Type::None)
        }
        _ => Err(Error::InvalidCode(Some(Location::from(typ.location)))),
    }
}

fn get_prarmeters(code: &pyast::Parameters) -> Result<(Vec<Symbol>, Vec<Type>), Error> {
    // todo: 这里无视掉了所有的参数，仅保留常规参数
    fn get_prarmeter(code: &pyast::Parameter) -> Result<(Symbol, Type), Error> {
        let typ = &code.annotation;
        let arg = Symbol::new(&code.arg);
        let typ = if let Some(typ) = typ {
            get_type(typ)?
        } else {
            Type::new()
        };
        Ok((arg, typ))
    }
    let r: Result<Vec<_>, Error> = code.args.iter().map(get_prarmeter).collect();
    let r = r?;
    let args = r.iter().map(|(arg, _)| arg.clone()).collect();
    let typs = r.iter().map(|(_, typ)| typ.clone()).collect();
    Ok((args, typs))
}

fn top_level_transform(code: &pyast::Statement) -> Result<AST, Error> {
    let node = &code.node;
    match node {
		/*
        pyast::StatementType::Assign {
			targets,
			value
		} => {
            todo!()
        }*/
        pyast::StatementType::ClassDef {
            name,
            body,
            bases,
            keywords,
            decorator_list,
        } => {
            todo!()
        }
        pyast::StatementType::FunctionDef {
            is_async,
            name,
            args,
            body,
            decorator_list,
            returns,
        } => {
            if *is_async || decorator_list.len() != 1 || body.len() != 1 {
                return invalid_code_error(code.location);
            }
            let (args, funtype) = get_prarmeters(args)?;
            let body = body.get(0).unwrap();
            let body = stmt_transform(body)?;
            let rettype = if let Some(typ) = returns {
                get_type(typ)?
            } else {
                Type::new()
            };
            let r = AST::Function(Handle::new(Function {
                name: Symbol::new(name),
                location: Location::from(code.location),
                args,
                funtype,
                rettype,
                body,
            }));
            Ok(r)
        }
        _ => invalid_code_error(code.location),
    }
}

pub fn ast_transform(program: pyast::Program) -> Result<AST, Error> {
    if program.statements.len() != 1 {
        Err(Error::InvalidCode(None))?;
    }
    let program = program.statements.get(0).unwrap();
    top_level_transform(program)
}
