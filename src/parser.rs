use crate::lexer::{Lexer, Token, TokenInfo};
use crate::ast::AstNode;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token_info: TokenInfo,
    in_condition: bool,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token_info = lexer.get_next_token();
        Parser {
            lexer,
            current_token_info,
            in_condition: false,
        }
    }

    fn eat(&mut self, token: Token) {
        if self.current_token_info.token == token {
            self.current_token_info = self.lexer.get_next_token();
        } else {
            panic!(
                "Expected {:?}, got {:?} at line {}, column {}",
                token,
                self.current_token_info.token,
                self.current_token_info.line,
                self.current_token_info.column
            );
        }
    }

    pub fn parse(&mut self) -> AstNode {
        let mut nodes = vec![];

        while self.current_token_info.token != Token::Eof {
            nodes.push(self.statement());
        }

        AstNode::Program(nodes)
    }

    fn statement(&mut self) -> AstNode {
        match self.current_token_info.token {
            Token::Identifier(_) => self.assignment_statement(),
            Token::Output => self.output_statement(),
            Token::If => self.if_statement(),
            Token::Loop => self.loop_statement(),
            _ => panic!(
                "Unexpected token: {:?} at line {}, column {}",
                self.current_token_info.token,
                self.current_token_info.line,
                self.current_token_info.column
            ),
        }
    }

    fn assignment_statement(&mut self) -> AstNode {
        if let Token::Identifier(name) = self.current_token_info.token.clone() {
            self.eat(Token::Identifier(name.clone()));
            self.eat(Token::Assign);
            let expr = self.expr();
            AstNode::Assignment(name, Box::new(expr))
        } else {
            panic!(
                "Expected identifier, got {:?} at line {}, column {}",
                self.current_token_info.token,
                self.current_token_info.line,
                self.current_token_info.column
            );
        }
    }

    fn output_statement(&mut self) -> AstNode {
        self.eat(Token::Output);
        let expr = self.expr();
        AstNode::Output(Box::new(expr))
    }

    fn if_statement(&mut self) -> AstNode {
        self.eat(Token::If);
        self.in_condition = true;
        let condition = self.boolean_expr();
        self.in_condition = false;
        self.eat(Token::Then);
        let mut true_branch = vec![];

        while self.current_token_info.token != Token::Else && self.current_token_info.token != Token::EndIf {
            true_branch.push(self.statement());
        }

        let false_branch = if self.current_token_info.token == Token::Else {
            self.eat(Token::Else);
            let mut false_branch = vec![];
            while self.current_token_info.token != Token::EndIf {
                false_branch.push(self.statement());
            }
            false_branch
        } else {
            vec![]
        };

        self.eat(Token::EndIf);

        AstNode::If(Box::new(condition), true_branch, false_branch)
    }

    fn loop_statement(&mut self) -> AstNode {
        self.eat(Token::Loop);
        self.eat(Token::While);
        self.in_condition = true;
        let condition = self.boolean_expr();
        self.in_condition = false;
        let mut body = vec![];

        while self.current_token_info.token != Token::EndLoop {
            body.push(self.statement());
        }

        self.eat(Token::EndLoop);

        AstNode::Loop(Box::new(condition), body)
    }

    fn boolean_expr(&mut self) -> AstNode {
        let mut node = self.expr();

        while matches!(
            self.current_token_info.token,
            Token::Assign
                | Token::NotEqual
                | Token::GreaterThan
                | Token::GreaterThanOrEqual
                | Token::LessThan
                | Token::LessThanOrEqual
                | Token::And
                | Token::Or
        ) {
            let token = self.current_token_info.token.clone();
            self.eat(token.clone());
            node = AstNode::BinOp(Box::new(node), token, Box::new(self.expr()));
        }

        node
    }

    fn expr(&mut self) -> AstNode {
        let mut node = self.term();

        while self.current_token_info.token == Token::Plus || self.current_token_info.token == Token::Minus {
            let token = self.current_token_info.token.clone();
            if token == Token::Plus {
                self.eat(Token::Plus);
            } else {
                self.eat(Token::Minus);
            }
            node = AstNode::BinOp(Box::new(node), token, Box::new(self.term()));
        }

        node
    }

    fn term(&mut self) -> AstNode {
        let mut node = self.factor();

        while matches!(
            self.current_token_info.token,
            Token::Star | Token::Slash | Token::Mod | Token::Div
        ) {
            let token = self.current_token_info.token.clone();
            match token {
                Token::Star => self.eat(Token::Star),
                Token::Slash => self.eat(Token::Slash),
                Token::Mod => self.eat(Token::Mod),
                Token::Div => self.eat(Token::Div),
                _ => {}
            }
            node = AstNode::BinOp(Box::new(node), token, Box::new(self.factor()));
        }

        node
    }

    fn factor(&mut self) -> AstNode {
        match self.current_token_info.token {
            Token::Number(value) => {
                self.eat(Token::Number(value));
                AstNode::Number(value)
            }
            Token::String(ref value) => {
                let value = value.clone();
                self.eat(Token::String(value.clone()));
                AstNode::String(value)
            }
            Token::Identifier(ref name) => {
                let name = name.clone();
                self.eat(Token::Identifier(name.clone()));
                AstNode::Identifier(name)
            }
            Token::LParen => {
                self.eat(Token::LParen);
                let node = self.expr();
                self.eat(Token::RParen);
                node
            }
            _ => panic!(
                "Unexpected token: {:?} at line {}, column {}",
                self.current_token_info.token,
                self.current_token_info.line,
                self.current_token_info.column
            ),
        }
    }
}
