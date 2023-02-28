use crate::token::{Token, TokenType};

pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Variable(VariableExpr),
    Assign(AssignExpr),
}

pub struct AssignExpr {
    /// left value token
    pub lvar: Token,
    /// right value expression
    pub value: Box<Expr>,
}
pub struct VariableExpr {
    pub var: Token,
}

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub expression: Box<Expr>,
}

pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

pub struct LiteralExpr {
    pub token: Token,
}

/// extract the value from a literal expression
impl LiteralExpr {
    pub fn get_literal_value(&self) -> LiteralValue {
        match self.token.r#type {
            TokenType::String => LiteralValue::Str(self.token.lexeme.to_owned()),
            TokenType::Number => {
                let num = self.token.lexeme.parse::<f64>().unwrap();
                LiteralValue::Num(num)
            }
            TokenType::True => LiteralValue::Bool(true),
            TokenType::False => LiteralValue::Bool(false),

            _ => {
                unreachable!()
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralValue {
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
}

impl std::fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralValue::Num(num) => write!(f, "{}", num),
            LiteralValue::Str(str) => write!(f, "{}", str),
            LiteralValue::Bool(b) => write!(f, "{}", b),
            LiteralValue::Nil => write!(f, "nil"),
        }
    }
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Binary(binary) => binary.to_string(),
            Expr::Unary(unary) => unary.to_string(),
            Expr::Grouping(grouping) => grouping.to_string(),
            Expr::Literal(literal) => literal.to_string(),
            Expr::Variable(var) => var.to_string(),
            Expr::Assign(assign) => assign.to_string(),
        }
    }
}

impl ToString for UnaryExpr {
    fn to_string(&self) -> String {
        format!(
            "({} {})",
            self.operator.lexeme.clone(),
            self.expression.to_string()
        )
    }
}

impl ToString for BinaryExpr {
    fn to_string(&self) -> String {
        format!(
            "({} {} {})",
            self.operator.lexeme.clone(),
            self.left.to_string(),
            self.right.to_string()
        )
    }
}

impl ToString for GroupingExpr {
    fn to_string(&self) -> String {
        format!("(grouping {})", self.expression.to_string())
    }
}

impl ToString for LiteralExpr {
    fn to_string(&self) -> String {
        self.token.lexeme.clone()
    }
}

impl ToString for VariableExpr {
    fn to_string(&self) -> String {
        self.var.lexeme.clone()
    }
}
impl ToString for AssignExpr {
    fn to_string(&self) -> String {
        format!("{} = {}", self.lvar.lexeme.clone(), self.value.to_string())
    }
}
#[test]
fn expression_to_string() {
    let literal_114 = LiteralExpr {
        token: Token::new(crate::token::TokenType::Number, "114".to_string(), 1),
    };
    let literal_514 = LiteralExpr {
        token: Token::new(crate::token::TokenType::Number, "514".to_string(), 1),
    };
    let token_plus = Token::new(crate::token::TokenType::Plus, "+".to_string(), 1);

    //    +
    //  /   \
    // 114  514
    let binary = BinaryExpr {
        left: Box::new(Expr::Literal(literal_114)),
        operator: token_plus.clone(),
        right: Box::new(Expr::Literal(literal_514)),
    };
    let expr = Expr::Binary(binary);

    let correct_string = String::from("(+ 114 514)");
    assert_eq!(expr.to_string(), correct_string);

    let unary = Expr::Unary(UnaryExpr {
        operator: token_plus.clone(),
        expression: Box::new(Expr::Literal(LiteralExpr {
            token: Token::new(crate::token::TokenType::Number, "514".to_string(), 1),
        })),
    });

    let complicated = Expr::Binary(BinaryExpr {
        left: Box::new(expr),
        operator: token_plus,
        right: Box::new(unary),
    });
    assert_eq!(complicated.to_string(), "(+ (+ 114 514) (+ 514))")
}

pub enum Value {
    Literal(LiteralValue),
    Variable(),
}
