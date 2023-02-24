use crate::{
    error::RuntimeError,
    expression::{BinaryExpr, Expr, LiteralExpr, LiteralValue, UnaryExpr},
    lox::Lox,
    parser::Parser,
    scanner::Scanner,
    token::TokenType,
};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn evaluate(&self, expr: &Expr) -> Result<Option<LiteralValue>, RuntimeError> {
        match expr {
            Expr::Binary(binary) => self.evaluate_binary(binary).map(|value| Some(value)),
            Expr::Unary(unary) => self.evaluate_unary(unary).map(|value| Some(value)),
            Expr::Grouping(grouping) => self.evaluate(&grouping.expression),
            Expr::Literal(literal) => Ok(Some(literal.get_literal_value())),
        }
    }
}

impl Interpreter {
    fn evaluate_unary(&self, expr: &UnaryExpr) -> Result<LiteralValue, RuntimeError> {
        if let Some(right) = self.evaluate(&expr.expression)? {
            match expr.operator.r#type {
                TokenType::Minus => {
                    if let LiteralValue::Num(num) = right {
                        Ok(LiteralValue::Num(-num))
                    } else {
                        Err(RuntimeError::new(format!(
                            "Operand must be number, not `{:?}`",
                            right
                        )))
                    }
                }
                TokenType::Bang => {
                    let truthy = self.is_truthy(&right);
                    Ok(LiteralValue::Bool(!truthy))
                }
                _ => Err(RuntimeError::new(format!(
                    "Invalid unary operator `{}`",
                    expr.operator.lexeme
                ))),
            }
        } else {
            Err(RuntimeError::new(format!(
                "Expression {} has no value.",
                expr.expression.to_string()
            )))
        }
    }

    fn evaluate_binary(&self, expr: &BinaryExpr) -> Result<LiteralValue, RuntimeError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
        let op_type = expr.operator.r#type;
        match (left, right, op_type) {
            // divided by zero
            // WARN. floating-point types cannot be used in patterns
            // this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
            (Some(_), Some(LiteralValue::Num(0.0)), TokenType::Slash) => Err(RuntimeError::new(
                "Divided by zero is not allowed.".to_string(),
            )),
            // evaluate numbers
            (
                Some(LiteralValue::Num(left_num)),
                Some(LiteralValue::Num(right_num)),
                TokenType::Plus
                | TokenType::Minus
                | TokenType::Slash
                | TokenType::Star
                | TokenType::EqualEqual
                | TokenType::BangEqual
                | TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual,
            ) => Ok(match op_type {
                TokenType::Plus => LiteralValue::Num(left_num + right_num),
                TokenType::Minus => LiteralValue::Num(left_num - right_num),
                TokenType::Slash => LiteralValue::Num(left_num / right_num),
                TokenType::Star => LiteralValue::Num(left_num * right_num),
                TokenType::EqualEqual => LiteralValue::Bool(left_num == right_num),
                TokenType::BangEqual => LiteralValue::Bool(left_num != right_num),
                TokenType::Greater => LiteralValue::Bool(left_num > right_num),
                TokenType::GreaterEqual => LiteralValue::Bool(left_num >= right_num),
                TokenType::Less => LiteralValue::Bool(left_num < right_num),
                TokenType::LessEqual => LiteralValue::Bool(left_num <= right_num),
                _ => unreachable!(),
            }),
            // string concat
            (
                Some(LiteralValue::Str(left_str)),
                Some(LiteralValue::Str(right_str)),
                TokenType::Plus,
            ) => Ok(LiteralValue::Str(format!("{}{}", left_str, right_str))),
            // left_expr has no value
            (None, Some(_), _) => Err(RuntimeError::new(format!(
                "Expression `{}` has no value.",
                expr.left.to_string(),
            ))),
            // right_expr has no value
            (Some(_), None, _) => Err(RuntimeError::new(format!(
                "Expression `{}` has no value.",
                expr.right.to_string(),
            ))),
            // both no value
            (None, None, _) => Err(RuntimeError::new(format!(
                "Expression `{}` and `{}` has no value.",
                expr.left.to_string(),
                expr.right.to_string(),
            ))),
            (_, _, _) => Err(RuntimeError::new(format!(
                "Expression `{}` can not be interpreted.",
                expr.to_string()
            ))),
        }
    }
}

/// util methods
impl Interpreter {
    fn is_truthy(&self, expr: &LiteralValue) -> bool {
        match expr {
            LiteralValue::Num(num) if *num == 0.0 => false,
            LiteralValue::Str(str) if str == "" => false,
            LiteralValue::Bool(b) => *b,
            _ => true,
        }
    }
}
#[test]
fn test_evaluate_unary() {
    let data = vec![
        ("!true", LiteralValue::Bool(false)),
        ("!false", LiteralValue::Bool(true)),
        ("!!true", LiteralValue::Bool(true)),
        ("!!false", LiteralValue::Bool(false)),
        ("-1", LiteralValue::Num(-1.0)),
        ("-8.1", LiteralValue::Num(-8.1)),
        ("\"a string\"", LiteralValue::Str(String::from("a string"))),
        ("\"-1\"", LiteralValue::Str(String::from("-1"))),
    ];

    for (input, should_be) in data {
        let mut scanner = Scanner::new(String::from(input));
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        let interpreter = Interpreter {};
        assert_eq!(should_be, interpreter.evaluate(&expr).unwrap().unwrap());
    }
}

#[test]
fn test_evaluate_binary() {
    let data = vec![
        ("1 + 2", LiteralValue::Num(3.)),
        ("1 / 2", LiteralValue::Num(1f64 / 2f64)),
        ("2 * 2", LiteralValue::Num(2. * 2.)),
        ("1 - 2", LiteralValue::Num(1. - 2.)),
        ("1>2", LiteralValue::Bool(false)),
        ("2>1.2", LiteralValue::Bool(true)),
        ("2 >= 2.1", LiteralValue::Bool(false)),
        ("2 <= 2.1", LiteralValue::Bool(true)),
        ("2 < 2.0", LiteralValue::Bool(false)),
        ("2 <= 2.0", LiteralValue::Bool(true)),
        ("0-8.1", LiteralValue::Num(-8.1)),
        (
            "\"one\" + \"two\"",
            LiteralValue::Str(String::from("onetwo")),
        ),
    ];

    for (input, should_be) in data {
        let mut scanner = Scanner::new(String::from(input));
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        let interpreter = Interpreter {};
        assert_eq!(should_be, interpreter.evaluate(&expr).unwrap().unwrap());
    }
}
