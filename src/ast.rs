use crate::lexer::Token;

#[derive(Debug)]
pub enum AstNode {
    Program(Vec<AstNode>),
    Assignment(String, Box<AstNode>),
    Output(Box<AstNode>),
    If(Box<AstNode>, Vec<AstNode>, Vec<AstNode>),
    Loop(Box<AstNode>, Vec<AstNode>),
    BinOp(Box<AstNode>, Token, Box<AstNode>),
    Number(i64),
    String(String),
    Identifier(String),
}
