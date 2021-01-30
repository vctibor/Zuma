/// This module takes Zuma AST,
/// evaluates all language constructs like variables, functions, loops, etc.,
/// and returns svg_generator's SVG representation.

use crate::parsing::ast as ast;
use crate::svg_generator as svg;

use std::collections::HashMap;
use std::boxed::Box;

use maplit::hashmap;
use anyhow::{Result, Error, anyhow};

enum Type {
    Number,
    Point,
    Color
}

use crate::evaluation::Type::*;

struct Function {
    //args: HashMap<String, Arg>,
    eval: Box<dyn FnOnce(ast::FunctionCall) -> Result<Vec<svg::Element>>>
}

struct Arg {
    r#type: Type,
    required: bool
}

/// TODO: Don't panic! Return errors.
pub fn evaluate(zuma_doc: ast::Document) -> svg::Document {

    /*
    // declaration of known functions, basically "stdlib"
    let functions: HashMap<String, Function> = hashmap!{
        "line".to_owned() =>  Function {
            args: hashmap!{
                "start".to_owned() => Arg { r#type: Point,  required: true },
                "end".to_owned()   => Arg { r#type: Point,  required: true },
                "color".to_owned() => Arg { r#type: Color,  required: false },
                "width".to_owned() => Arg { r#type: Number, required: false },
            },

            eval: Box::new(move |fc: ast::FunctionCall| {
                panic!()
            })
        }
    };
    */

    let functions: HashMap<String, Function> = hashmap!{
        "line".to_owned() =>  Function {
            eval: Box::new(eval_line)
        }
    };


    let mut document = svg::Document::new();

    for row in zuma_doc.rows {

        let function_call: ast::FunctionCall = row;    // later `row` will be enum
        let mut res = handle_function_call(function_call, &functions);
        document = document.add_many(&mut res);
        
    };

    document
}

fn handle_function_call(function_call: ast::FunctionCall, functions: &HashMap<String, Function>) -> Vec<svg::Element> {
    
    let name = function_call.name.clone();
    
    println!("Function {:?}", name);


    let function = functions.get(&name);

    if function.is_none() {
        panic!("Unknown function {}", &function_call.name);
    }

    let function = function.unwrap();

    //let all_accepted_args: Vec<String> = function.args.keys().map(|key| key.clone()).collect();
    //println!("Accepted args {:?}", all_accepted_args);
    //let required_args: Vec<String> = function.args.iter().filter(|(_, val)| val.required).map(|(key, _)| key.clone()).collect();
    //println!("Required args {:?}", required_args);
    //let provided_args: Vec<String> = function_call.args.iter().map(|arg| arg.name.clone()).collect();   
    //println!("Provided args {:?}", provided_args);

    // TODO: error if required argument isn't provided
    
    // TODO: error if some provided argument is unknown
    
    // TODO: we also need to match types ! ! !

    panic!();
}

fn eval_line(fc: ast::FunctionCall) -> Result<Vec<svg::Element>> {

    let mut args = fc.args;
    
    let start = get_arg(&mut args, "start", Type::Point)?;
    let end = get_arg(&mut args, "end", Type::Point)?;

    let start = get_arg(&mut args, "color", Type::Color);
    let start = get_arg(&mut args, "width", Type::Number);


    Ok(vec!())
}

fn get_arg(args: &mut Vec<ast::Arg>, name: &str, r#type: Type) -> Result<ast::Arg> {

    let index = args.iter().position(|arg| arg.name == name);

    if index.is_none() {
        return Err(anyhow!("Missing required argument `start`."));
    }
    
    let index = index.unwrap();

    let arg = args.remove(index);

    Ok(arg)
}