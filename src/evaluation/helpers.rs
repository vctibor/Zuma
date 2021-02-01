use crate::parsing::ast as ast;
use anyhow::{Result, anyhow};


pub fn get_arg(args: &mut Vec<ast::Arg>, name: &str) -> Result<ast::Arg> {

    let index = args.iter().position(|arg| arg.name == name);

    if let Some(index) = index {
        return Ok(args.remove(index));
    }

    Err(anyhow!("Missing required argument `{}`.", name))
}


pub fn get_string(arg: ast::Arg) -> Result<String> {
    match arg.value {
        ast::Value::String(s) => Ok(s),
        _ => Err(anyhow!("Wrong type."))
    }
}

pub fn get_point(arg: ast::Arg) -> Result<ast::Point> {
    match arg.value {
        ast::Value::Point(p) => Ok(p),
        _ => Err(anyhow!("Wrong type."))
    }
}

pub fn get_color(arg: ast::Arg) -> Result<ast::Color> {
    match arg.value {
        ast::Value::Color(c) => Ok(c),
        _ => Err(anyhow!("Wrong type."))
    }
}

pub fn get_number(arg: ast::Arg) -> Result<f32> {
    match arg.value {
        ast::Value::Number(n) => Ok(n),
        _ => Err(anyhow!("Wrong type."))
    }
}



