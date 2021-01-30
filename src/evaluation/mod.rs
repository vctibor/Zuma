/// This module takes Zuma AST,
///   evaluates all language constructs like variables, functions, loops, etc.,
///   and returns svg_generator's SVG representation.

use crate::parsing::ast as ast;
use crate::svg_generator as svg;

use std::collections::HashMap;

use maplit::hashmap;

#[derive(Debug, PartialEq, Clone)]
enum Type {
    Number,
    Point,
    Color
}

use crate::evaluation::Type::*;

#[derive(Debug, PartialEq, Clone)]
struct Function {
    args: HashMap<String, Arg>
}

#[derive(Debug, PartialEq, Clone)]
struct Arg {
    r#type: Type,
    required: bool
}


/// TODO: Don't panic! Return errors.
pub fn evaluate(zuma_doc: ast::Document) -> svg::Document {

    let FUNCTIONS: HashMap<&str, Function> = hashmap!{
        "line" =>  Function { args: hashmap!{
            "start".to_owned() => Arg { r#type: Point,  required: true },
            "end".to_owned()   => Arg { r#type: Point,  required: true },
            "color".to_owned() => Arg { r#type: Color,  required: false },
            "width".to_owned() => Arg { r#type: Number, required: false },
        }}
    };


    let mut document = svg::Document::new();

    for row in zuma_doc.rows {

        let function_call: ast::FunctionCall = row;    // later `row` will be enum

        /*
        for element in handle_function_call(function_call) {
            document.add(element);
        }
        */
    };

    document
}


/*
fn handle_function_call(function_call: ast::FunctionCall) -> Vec<svg::Element> {
    
    let function = FUNCTIONS.get(function_call.name);

    if function.is_none() {
        panic!("Unknown function {}", &function_call.name);
    }

    let function = function.unwrap();

    // error if required argument isn't provided
    for arg_name in function.args.keys() {
        let arg = function.args.get(arg_name).unwrap();
        if arg.required && !function_call.args.contains(&arg_name) {
            panic!("Required argument {} wasn't provided.", arg_name);
        }
    }

    // error if some provided argument is unknown
    for arg in function_call.args {
        if !function.args.contains_key(&arg.name) {
            panic!("Unknown argument {} provided.", arg.name);
        }
    }

    panic!();
}
*/