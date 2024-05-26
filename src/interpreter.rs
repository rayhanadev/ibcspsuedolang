use crate::ast::AstNode;
use crate::lexer::Token;
use std::collections::HashMap;

pub struct Interpreter {
    variables: HashMap<String, i64>,
    in_condition: bool,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            in_condition: false,
        }
    }

    pub fn interpret(&mut self, node: &AstNode) {
        match node {
            AstNode::Program(statements) => {
                for statement in statements {
                    self.interpret(statement);
                }
            }
            AstNode::Assignment(name, expr) => {
                let value = self.eval_expr(expr);
                self.variables.insert(name.clone(), value);
            }
            AstNode::Output(expr) => {
                if let AstNode::String(value) = &**expr {
                    println!("{}", value);
                } else {
                    let value = self.eval_expr(expr);
                    println!("{}", value);
                }
            }
            AstNode::If(condition, true_branch, false_branch) => {
                self.in_condition = true;
                let cond_value = self.eval_expr(condition);
                self.in_condition = false;
                if cond_value != 0 {
                    for statement in true_branch {
                        self.interpret(statement);
                    }
                } else {
                    for statement in false_branch {
                        self.interpret(statement);
                    }
                }
            }
            AstNode::Loop(condition, body) => {
                while self.eval_expr(condition) != 0 {
                    for statement in body {
                        self.interpret(statement);
                    }
                }
            }
            _ => panic!("Unknown AST node"),
        }
    }

    fn eval_expr(&mut self, node: &AstNode) -> i64 {
        match node {
            AstNode::Number(value) => *value,
            AstNode::String(_) => panic!("Cannot evaluate string as number"),
            AstNode::Identifier(name) => *self.variables.get(name).expect("Undefined variable"),
            AstNode::BinOp(left, op, right) => {
                let left_val = self.eval_expr(left);
                let right_val = self.eval_expr(right);
                match op {
                    Token::Plus => left_val + right_val,
                    Token::Minus => left_val - right_val,
                    Token::Star => left_val * right_val,
                    Token::Slash => left_val / right_val,
                    Token::Mod => left_val % right_val,
                    Token::Assign if self.in_condition => (left_val == right_val) as i64,
                    Token::NotEqual => (left_val != right_val) as i64,
                    Token::GreaterThan => (left_val > right_val) as i64,
                    Token::GreaterThanOrEqual => (left_val >= right_val) as i64,
                    Token::LessThan => (left_val < right_val) as i64,
                    Token::LessThanOrEqual => (left_val <= right_val) as i64,
                    Token::And => ((left_val != 0) && (right_val != 0)) as i64,
                    Token::Or => ((left_val != 0) || (right_val != 0)) as i64,
                    _ => panic!("Unknown binary operator"),
                }
            }
            _ => panic!("Unknown expression"),
        }
    }
}
