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
pub use self::graphics::*;

use crate::parsing::ast as ast;
use crate::stack::Stack;

use anyhow::{Result, anyhow};

pub struct Interpreter {
    constants: Stack<String, ast::Value>,
    
    /// User-defined procedures
    procedures: Stack<String, ast::UserProcedure>,
}

impl Interpreter {
        
    pub fn new() -> Interpreter {
        Interpreter {
            constants: Stack::new(),
            procedures: Stack::new(),
        }
    }

    pub fn add_frame(&mut self) {
        self.constants.add_frame();
        self.procedures.add_frame();
    }

    pub fn pop_frame(&mut self) {
        self.constants.pop_frame();
        self.procedures.pop_frame();
    }

    pub fn interpret(&mut self, zuma: ast::Document) -> Result<Graphics> {
        let mut graphics = Graphics::new();
        graphics = self.handle_expressions(zuma.expressions, graphics)?;
        Ok(graphics)
    }

    fn handle_expressions(&mut self,
        expressions: Vec<ast::Expression>,
        mut graphics: Graphics     // it should be possible to remove this, right?
        )
        -> Result<Graphics>
    {
        use crate::parsing::ast::Expression::*;

        self.add_frame();

        for expr in expressions {

            match expr {

                FunctionCall(fc) => {

                    let name = fc.name.clone();

                    let user_procedure = self.procedures.get(name).cloned();

                    graphics = if let Some(user_defined) = user_procedure {

                        use crate::parsing::ast::ProcArg::*;

                        // add new frame to the stack
                        self.add_frame();

                        // add default values from procedure definition
                        for arg in user_defined.args {
                            match arg {                                
                                Optional(optional_arg) => {       
                                    // default argument values provided as operation should be evaluated when procedure is defined not called, and then stored as values                             
                                    let evaluated = get_value(&optional_arg.default_value, &self.constants)?;
                                    self.constants.add_to_current_frame(optional_arg.name, evaluated);
                                },
                                Required(_) => {},
                            }
                        }

                        // add values from call
                        for arg in fc.args {
                            let evaluated = get_value(&arg.value, &self.constants)?;
                            self.constants.add_to_current_frame(arg.name, evaluated);
                        }

                        // evaluate procedure
                        let graphics = self.handle_expressions(user_defined.body.expressions.clone(), graphics)?;

                        // pop frame
                        self.pop_frame();

                        graphics
                    }
                    else
                    {
                        graphics.add_many(
                            self.handle_function_call(fc)?
                        )
                    }
                },
                
                ConstantDeclaration(c) => {
                    use crate::parsing::OperationInput::*;
                    let resulting_constant = match c.value {
                        Literal(value) => value,
                        Constant(name) => get_constant(&name, &self.constants)?,
                        Operation(op) => eval_operation(op, &self.constants)?,
                    };

                    self.constants.add_to_current_frame(c.name, resulting_constant);
                },
                
                Scope(s) => {
                    graphics = self.handle_expressions(s.expressions, graphics)?;
                },

                IfStatement(if_statement) => {

                    let mut if_statement = if_statement.clone();

                    let eval_cond = get_value(&if_statement.condition, &self.constants)?.get_bool()?;
                    
                    let expr = if eval_cond {
                        if_statement.positive.as_mut().expressions.clone()
                    } else {
                        if_statement.negative.as_mut().expressions.clone()
                    };

                    graphics = self.handle_expressions(expr, graphics)?;
                },

                ForLoop(for_loop) => {

                    let ast::ForLoop { index_name, starting_value, step, final_value, scope } = for_loop;

                    let starting_value = get_value(&starting_value, &self.constants)?.get_number()?;
                    let step = get_value(&step, &self.constants)?.get_number()?;
                    let final_value = get_value(&final_value, &self.constants)?.get_number()?;

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
                        self.constants.add_to_current_frame(index_name.clone(), ast::Value::Number(current_index_value));
                        graphics = self.handle_expressions(scope.expressions.clone(), graphics)?;                        
                        current_index_value += step;
                    }
                },

                UserProcedure(user_proc) => {
                    self.procedures.add_to_current_frame(user_proc.name.clone(), user_proc);
                }
            }
        }

        self.pop_frame();
        Ok(graphics)
    }

    fn handle_function_call(&mut self, function_call: ast::FunctionCall)
        -> Result<Vec<GraphicNode>>
    {
        let ast::FunctionCall { name, args } = function_call;
        let args = helpers::create_arg_map(args, &self.constants)?;

        match name.as_str() {
            "line" => stdlib::line(args, &self.constants),
            "rectangle" => stdlib::rectangle(args, &self.constants),
            "text" => stdlib::text(args, &self.constants),
            "ellipse" => stdlib::ellipse(args, &self.constants),
            "polygon" => stdlib::polygon(args, &self.constants),
            unknown => Err(anyhow!("Unknown function {}!", unknown)),
        }
    }
}
