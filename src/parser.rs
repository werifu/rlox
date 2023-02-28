// program        → declaration * EOF ;
// declaration    → varDecl
//                | statement ;
// varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
// statement      → exprStmt
//                | printStmt
//                | block;
// block          → "{" declaration* "}" ;
// exprStmt       → expression ";" ;
// printStmt      → "print" expression ";" ;
// expression     → assignment ;
// assignment     → IDENTIFIRE "=" assignment
//                | equality;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")"
//                | IDENTIFIER ;

use crate::{
    error::ParseError,
    expression::{
        AssignExpr, BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr, VariableExpr,
    },
    statement::{Block, ExprStmt, PrintStmt, Stmt, VarDecStmt},
    token::Token,
    token::TokenType,
};

pub struct Parser {
    pub tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = vec![];
        while !self.is_at_end() {
            // TODO: engage all the parse errors
            match self.declaration() {
                Ok(statement) => statements.push(statement),
                Err(_) => self.synchronize(),
            }
        }
        Ok(statements)
    }

    pub fn all_parsed(&self) -> bool {
        self.current == self.tokens.len() - 1
    }

    // Make test easier
    pub fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }
}

impl Parser {
    // block          → "{" declaration* "}" ;
    fn block(&mut self) -> Result<Stmt, ParseError> {
        let mut stmts = vec![];
        // not } or end meaning still in the block
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            stmts.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace);

        Ok(Stmt::Block(Block::new(stmts)))
    }

    // declaration    → varDecl
    //                | statement ;
    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.token_type_match(&vec![TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    // varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let var_name = self.consume(TokenType::Identifier)?.lexeme.clone();
        let mut expr: Option<Expr> = None;
        if self.token_type_match(&vec![TokenType::Equal]) {
            expr = Some(self.expression()?);
            self.consume(TokenType::Semicolon)?;
        }
        Ok(Stmt::Var(VarDecStmt::new(var_name, expr)))
    }

    /// statement      → exprStmt
    ///                | printStmt
    ///                | block ;
    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.token_type_match(&vec![TokenType::Print]) {
            self.print_stmt()
        } else if self.token_type_match(&vec![TokenType::LeftBrace]) {
            self.block()
        } else {
            self.expr_stmt()
        }
    }

    /// printStmt      → "print" expression ";" ;
    fn print_stmt(&mut self) -> Result<Stmt, ParseError> {
        let stmt = self
            .expression()
            .map(|expr| Stmt::Print(PrintStmt::new(expr)))?;
        self.consume(TokenType::Semicolon)?;
        Ok(stmt)
    }

    /// exprStmt       → expression ";" ;
    fn expr_stmt(&mut self) -> Result<Stmt, ParseError> {
        let stmt = self
            .expression()
            .map(|expr| Stmt::Expr(ExprStmt::new(expr)))?;
        self.consume(TokenType::Semicolon)?;
        Ok(stmt)
    }

    /// expression     → equality ;
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.equality()?;
        // assignment statement
        if self.token_type_match(&vec![TokenType::Equal]) {
            let equals = self.previous().to_owned();
            let value = self.assignment()?;
            if let Expr::Variable(var_expr) = expr {
                let token = var_expr.var;
                return Ok(Expr::Assign(AssignExpr {
                    lvar: token,
                    value: Box::new(value),
                }));
            }
            // TODO: more detail error
            return Err(ParseError::new(format!(
                "Invalid assignment target `{:?}`.",
                equals
            )));
        }
        Ok(expr)
    }
    /// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        let op_types = vec![TokenType::BangEqual, TokenType::EqualEqual];
        while self.token_type_match(&op_types) {
            let op = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        let op_types = vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];

        while self.token_type_match(&op_types) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    // term           → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        let op_types = vec![TokenType::Minus, TokenType::Plus];

        while self.token_type_match(&op_types) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    // factor         → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        let op_types = vec![TokenType::Slash, TokenType::Star];

        while self.token_type_match(&op_types) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    // unary          → ( "!" | "-" ) unary
    //                | primary ;
    fn unary(&mut self) -> Result<Expr, ParseError> {
        let op_types = vec![TokenType::Bang, TokenType::Minus];
        if self.token_type_match(&op_types) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            Ok(Expr::Unary(UnaryExpr {
                operator,
                expression: Box::new(right),
            }))
        } else {
            self.primary()
        }
    }

    // primary        → NUMBER | STRING | "true" | "false" | "nil"
    //                | "(" expression ")" ;
    //                | IDENTIFIER
    fn primary(&mut self) -> Result<Expr, ParseError> {
        let lit_types = vec![
            TokenType::False,
            TokenType::True,
            TokenType::Nil,
            TokenType::Number,
            TokenType::String,
        ];

        if self.token_type_match(&lit_types) {
            Ok(Expr::Literal(LiteralExpr {
                token: self.previous().clone(),
            }))
        } else if self.token_type_match(&vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen).unwrap();
            Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }))
        } else if self.token_type_match(&vec![TokenType::Identifier]) {
            Ok(Expr::Variable(VariableExpr {
                var: self.previous().clone(),
            }))
        } else {
            unreachable!()
        }
    }
}

impl Parser {
    /// returns self.tokens[current - 1]
    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }
    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().r#type == TokenType::Eof
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().r#type == token_type
        }
    }

    fn token_type_match(&mut self, types: &Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(*token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            let cur = self.peek();
            Err(ParseError::new(format!(
                "[line {}]Token type `{}` are expected, but got `{}`",
                cur.line, token_type, cur.lexeme
            )))
        }
    }
}

impl Parser {
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().r#type == TokenType::Semicolon {
                return;
            }

            // reach a new statement
            match self.peek().r#type {
                TokenType::Class
                | TokenType::Func
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {}
            };

            self.advance();
        }
    }
}
