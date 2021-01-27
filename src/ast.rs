#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}