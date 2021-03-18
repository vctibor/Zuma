use anyhow::{Result, anyhow};

#[derive(Debug, PartialEq, Clone)]
pub struct Document {
    pub expressions: Vec<Expression>
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    ConstantDeclaration(ConstantDeclaration),
    FunctionCall(FunctionCall),
    Scope(Scope),
    IfStatement(IfStatement),
    ForLoop(ForLoop),
    UserProcedure(UserProcedure),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Scope {
    pub expressions: Vec<Expression>
}

// STATEMENTS

#[derive(Debug, PartialEq, Clone)]
pub struct ForLoop {
    pub index_name: String,
    pub starting_value: OperationInput,
    pub step: OperationInput,
    pub final_value: OperationInput,
    pub scope: Scope,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement {
    pub condition: OperationInput,
    pub positive: Box<Scope>,
    pub negative: Box<Scope>,
}


// FUNCTIONS

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Arg>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Arg {
    pub name: String,
    pub value: OperationInput
}


// CONSTANTS

#[derive(Debug, PartialEq, Clone)]
pub struct ConstantDeclaration {
    pub name: String,
    pub value: OperationInput,
}


// ARITHMETIC AND LOGICAL OPERATIONS

#[derive(Debug, PartialEq, Clone)]
pub enum OperationInput {
    Literal(Value),
    Constant(String),
    Operation(Operation),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Operation {
    pub lh: Box<OperationInput>,
    pub rh: Box<OperationInput>,
    pub op: Operator
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    LessThan,
    GreaterThan,
    Equality,
    LogicalAnd,
    LogicalOr,
    //Negation,  // unary operator, requires different implementation
}

// LITERALS

/// Types which can be declared by literal in Zuma.
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f32),
    Point(Point),
    Color(Color),
    String(String),
    Bool(bool),

    // Arrays
    ArrayNumber(Vec<f32>),
    ArrayPoint(Vec<Point>),
    ArrayColor(Vec<Color>),
    ArrayText(Vec<String>),
    ArrayBool(Vec<bool>),
}

impl Value {
    pub fn get_string(self) -> Result<String> {
        match self {
            Value::String(s) => Ok(s),
            _ => Err(anyhow!("Wrong type."))
        }
    }
    
    pub fn get_point(self) -> Result<Point> {
        match self {
            Value::Point(p) => Ok(p),
            _ => Err(anyhow!("Wrong type."))
        }
    }
    
    pub fn get_color(self) -> Result<Color> {
        match self {
            Value::Color(c) => Ok(c),
            _ => Err(anyhow!("Wrong type."))
        }
    }
    
    pub fn get_number(self) -> Result<f32> {
        match self {
            Value::Number(n) => Ok(n),
            _ => Err(anyhow!("Wrong type."))
        }
    }
    
    pub fn get_bool(self) -> Result<bool> {
        match self {
            Value::Bool(b) => Ok(b),
            _ => Err(anyhow!("Wrong type."))
        }
    }

    pub fn get_bool_array(self) -> Result<Vec<bool>> {
        match self {
            Value::ArrayBool(b) => Ok(b),
            _ => Err(anyhow!("Wrong type."))
        }
    }

    pub fn get_text_array(self) -> Result<Vec<String>> {
        match self {
            Value::ArrayText(s) => Ok(s),
            _ => Err(anyhow!("Wrong type."))
        }
    }
    
    pub fn get_point_array(self) -> Result<Vec<Point>> {
        match self {
            Value::ArrayPoint(p) => Ok(p),
            _ => Err(anyhow!("Wrong type."))
        }
    }
    
    pub fn get_color_array(self) -> Result<Vec<Color>> {
        match self {
            Value::ArrayColor(c) => Ok(c),
            _ => Err(anyhow!("Wrong type."))
        }
    }
    
    pub fn get_number_array(self) -> Result<Vec<f32>> {
        match self {
            Value::ArrayNumber(n) => Ok(n),
            _ => Err(anyhow!("Wrong type."))
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Color {
    pub red: Box<OperationInput>,
    pub green: Box<OperationInput>,
    pub blue: Box<OperationInput>,
}


#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: Box<OperationInput>,
    pub y: Box<OperationInput>,
}

// USER DECLARED PROCEDURES

#[derive(Debug, PartialEq, Clone)]
pub struct UserProcedure {
    pub name: String,
    pub args: Vec<ProcArg>,
    pub body: Scope,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ProcArg {
    Optional(OptionalArg),
    Required(RequiredArg),
}

#[derive(Debug, PartialEq, Clone)]
pub struct OptionalArg {
    pub name: String,
    pub default_value: OperationInput,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RequiredArg {
    pub name: String,
    pub arg_type: ZumaType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ZumaType {
    Bool,
    Number,
    Point,
    Color,
    Text,
}