//! parsing -> type checking -> INTERPRETATION -> code generation
//!
//! This module takes Zuma AST,
//! evaluates all language constructs like variables, functions, loops, etc.,
//! and returns Graphics type representing result of interpretation:
//! simple dictionary containing values describing basic graphic primitives of SVG.

// Once we have type checking in separate layer we will be able to coearce any variable into string
// and that will drastically simplify implementation of this module.

mod graphics;
mod helpers;
mod stdlib;

pub use helpers::*;
pub use self::graphics::{GraphicNode, Graphics};

use crate::parsing::ast as ast;

use std::collections::HashMap;

use anyhow::{Result, anyhow};

pub fn interpret(zuma: ast::Document) -> Result<Graphics> {
        
    let mut graphics = Graphics::new();

    let constants = vec!();
    graphics = handle_expressions(zuma.expressions, graphics, &constants)?;

    //println!("{:?}", graphics);

    Ok(graphics)
}

fn handle_expressions(expressions: Vec<ast::Expression>,
                      mut graphics: Graphics,
                      upper_scope_constants: &Constants)
    -> Result<Graphics>
{
    use crate::parsing::ast::Expression::*;

    let mut local_consts: ConstantsMap = HashMap::new();

    for expr in expressions {

        // TODO: I know this is highly suboptimal, but until it starts to cause measurable
        //  performance impact, I am leaving it. Anyway, it would be nice to clean it up.
        let mut mut_upper = upper_scope_constants.clone();
        let mut all_constants = vec!();
        let local_consts_clone = local_consts.clone();
        all_constants.push(&local_consts_clone);
        all_constants.append(&mut mut_upper);

        match expr {

            FunctionCall(fc) => {
                graphics = graphics.add_many(
                    handle_function_call(fc, &all_constants)?
                );
            },
            
            ConstantDeclaration(c) => {
                use crate::parsing::OperationInput::*;
                let resulting_constant = match c.value {
                    Literal(value) => value,
                    Constant(name) => get_constant(&name, &all_constants)?,
                    Operation(op) => eval_operation(op, &all_constants)?,
                };

                local_consts.insert(c.name, resulting_constant);
            },
            
            Scope(s) => {
                graphics = handle_expressions(s.expressions, graphics, &all_constants)?;
            },

            IfStatement(if_statement) => {

                let mut if_statement = if_statement.clone();

                let eval_cond = get_value(&if_statement.condition, &all_constants)?.get_bool()?;
                
                let expr = if eval_cond {
                    if_statement.positive.as_mut().expressions.clone()
                } else {
                    if_statement.negative.as_mut().expressions.clone()
                };

                graphics = handle_expressions(expr, graphics, &all_constants)?;
            },

            ForLoop(for_loop) => {

                let ast::ForLoop { index_name, starting_value, step, final_value, scope } = for_loop;

                if step == 0.0 {
                    return Err(anyhow!("Step can't be zero!"));
                }

                let condition = if step > 0.0 {
                    |curr, fin| curr < fin
                } else {
                    |curr, fin| curr > fin
                };

                let mut current_index_value = starting_value;

                while condition(current_index_value, final_value) {


                    // TODO: deduplicate
                    let mut mut_upper = upper_scope_constants.clone();
                    let mut all_constants = vec!();
                    let mut local_consts_clone = local_consts.clone();

                    local_consts_clone.insert(index_name.clone(), ast::Value::Number(current_index_value));

                    all_constants.push(&local_consts_clone);
                    all_constants.append(&mut mut_upper);


                    

                    graphics = handle_expressions(scope.expressions.clone(), graphics, &all_constants)?;

                    local_consts.remove(&index_name);
                    
                    current_index_value += step;
                }

            }
        }       
    }

    Ok(graphics)
}

fn handle_function_call(function_call: ast::FunctionCall, consts: &Constants) -> Result<Vec<GraphicNode>>
{
    
    let ast::FunctionCall { name, args } = function_call;
    
    let stdlib = stdlib::stdlib();

    match stdlib.get(&name) {
        Some(fun) => {
            let args = helpers::create_arg_map(args, &consts)?;
            Ok((fun.eval)(args, consts)?)
        }

        None => Err(anyhow!("Unknown function {}", &name))
    }
}