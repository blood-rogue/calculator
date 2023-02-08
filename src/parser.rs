use std::{
    cmp::{Ord, Ordering},
    convert::TryFrom,
    error::Error,
    fmt,
    iter::Peekable,
    slice::Iter,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Plus,
    Dash,
    Star,
    Slash,
    RightParen,
    LeftParen,
    End,
    Number(f64),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Multiply,
    Divide,
    Subtract,
}

impl Operator {
    fn cmp_val(&self) -> usize {
        match self {
            Operator::Multiply => 5,
            Operator::Divide => 5,
            Operator::Add => 3,
            Operator::Subtract => 3,
        }
    }
}

impl TryFrom<Token> for Operator {
    type Error = String;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Plus => Ok(Operator::Add),
            Token::Star => Ok(Operator::Multiply),
            Token::Dash => Ok(Operator::Subtract),
            Token::Slash => Ok(Operator::Divide),
            _ => Err(format!("Can only convert operators. Got: {:?}", token)),
        }
    }
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp_val().cmp(&other.cmp_val()))
    }
}

impl Ord for Operator {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_val().cmp(&other.cmp_val())
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Binary(Operator, Box<Expression>, Box<Expression>),
    Number(f64),
}

impl Expression {
    fn eval(&mut self) -> f64 {
        match self {
            Expression::Number(n) => *n,
            Expression::Binary(Operator::Add, expr1, expr2) => expr1.eval() + expr2.eval(),
            Expression::Binary(Operator::Multiply, expr1, expr2) => expr1.eval() * expr2.eval(),
            Expression::Binary(Operator::Subtract, expr1, expr2) => expr1.eval() - expr2.eval(),
            Expression::Binary(Operator::Divide, expr1, expr2) => expr1.eval() / expr2.eval(),
        }
    }
}

#[derive(Debug)]
pub struct SyntaxError {
    message: String,
    level: String,
}

impl SyntaxError {
    fn new_parse_error(message: String) -> Self {
        SyntaxError {
            message,
            level: "Parse".to_string(),
        }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} Error {}", self.level, self.message)
    }
}

impl Error for SyntaxError {}

pub struct ClimbingParser<'a> {
    iter: &'a mut Peekable<Iter<'a, Token>>,
}

impl<'a> ClimbingParser<'a> {
    pub fn new(iter: &'a mut Peekable<Iter<'a, Token>>) -> Self {
        ClimbingParser { iter }
    }

    fn assert_next(&mut self, token: Token) -> Result<(), SyntaxError> {
        let next = self.iter.next();
        if let None = next {
            return Err(SyntaxError::new_parse_error(
                "Unexpected end of input".to_string(),
            ));
        }

        if *next.unwrap() != token {
            return Err(SyntaxError::new_parse_error(format!(
                "Expected {:?} actual {:?}",
                token,
                next.unwrap(),
            )));
        }

        Ok(())
    }

    fn primary(&mut self) -> Result<Expression, SyntaxError> {
        match self.iter.next().unwrap() {
            Token::LeftParen => {
                let expr = self.expression(0)?;
                self.assert_next(Token::RightParen)?;
                Ok(expr)
            }
            Token::Number(n) => Ok(Expression::Number(*n)),
            tok => Err(SyntaxError::new_parse_error(format!(
                "Unexpected token {:?}",
                tok
            ))),
        }
    }

    fn expression(&mut self, precedence: usize) -> Result<Expression, SyntaxError> {
        let mut expr = self.primary()?;
        while let Some(tok) = self.iter.peek() {
            if Token::End == **tok || **tok == Token::RightParen {
                break;
            }
            let operator = Operator::try_from(**tok).unwrap();
            if operator.cmp_val() < precedence {
                break;
            }
            self.iter.next();
            let inner_precedence = operator.cmp_val() + 1;
            let rhs = self.expression(inner_precedence)?;
            expr = Expression::Binary(operator, Box::new(expr), Box::new(rhs));
        }

        Ok(expr)
    }

    pub fn parse(&mut self) -> Result<Expression, SyntaxError> {
        let ast = self.expression(0)?;
        self.assert_next(Token::End)?;
        Ok(ast)
    }
}

pub fn eval(tokens: Vec<Token>) -> Result<f64, Box<dyn Error>> {
    let mut token_iter = tokens.iter().peekable();
    let mut parser = ClimbingParser::new(&mut token_iter);
    let result = parser.parse();
    match result {
        Ok(mut ast) => return Ok(ast.eval()),
        Err(e) => return Err(Box::new(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(eval(vec![Token::Number(8.0), Token::Star, Token::Number(8.0), Token::Plus, Token::LeftParen]).unwrap(), 64.0)
    }
}