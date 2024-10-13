use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(i32),
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Var(String),
    Assign(String, Box<Expr>),
    VarDecl(bool, String, Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    Mod,
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Expr {
        self.parse_statement()
    }

    fn current_token(&self) -> Token {
        self.tokens[self.pos].clone()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn parse_statement(&mut self) -> Expr {
        match self.current_token() {
            Token::ConstVar => self.parse_var_declaration(true),
            Token::Mutate => self.parse_var_declaration(false),
            _ => self.parse_assignment(),
        }
    }

    fn parse_var_declaration(&mut self, is_immut: bool) -> Expr {
        self.advance();
        if let Token::Ident(name) = self.current_token() {
            let var_name = name.clone();
            self.advance();
            if let Token::Equal = self.current_token() {
                self.advance();
                let value = self.parse_expr();
                Expr::VarDecl(is_immut, var_name, Box::new(value))
            } else {
                panic!("Expected '=' after variable name in declaration");
            }
        } else {
            panic!("Expected identifier after 'const' or 'mutate'");
        }
    }

    fn parse_assignment(&mut self) -> Expr {
        let expr = self.parse_expr();

        if let Token::Equal = self.current_token() {
            self.advance();
            if let Expr::Var(name) = expr {
                let value = self.parse_expr();
                return Expr::Assign(name, Box::new(value));
            } else {
                panic!("Invalid assignment target");
            }
        }

        expr
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Expr {
        let mut node = self.parse_factor();

        while matches!(self.current_token(), Token::Plus | Token::Minus) {
            let op = match self.current_token() {
                Token::Plus => BinOp::Plus,
                Token::Minus => BinOp::Minus,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_factor();
            node = Expr::Binary(Box::new(node), op, Box::new(right));
        }

        node
    }

    fn parse_factor(&mut self) -> Expr {
        let mut node = self.parse_primary();

        while matches!(self.current_token(), Token::Asterisk | Token::Slash | Token::Percent) {
            let op = match self.current_token() {
                Token::Asterisk => BinOp::Multiply,
                Token::Slash => BinOp::Divide,
                Token::Percent => BinOp::Mod,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_primary();
            node = Expr::Binary(Box::new(node), op, Box::new(right));
        }

        node
    }

    fn parse_primary(&mut self) -> Expr {
        match self.current_token() {
            Token::Number(value) => {
                self.advance();
                Expr::Number(value)
            }
            Token::Ident(ref name) => {
                let var_name = name.clone();
                self.advance();
                Expr::Var(var_name)
            }
            _ => panic!("Unexpected token: {:?}", self.current_token()),
        }
    }
}
