#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::num::ParseFloatError;

enum Token {
    Operator(Operator),
    Number(f64)
}

#[derive(Clone, Copy, PartialEq)]
enum Operator {
    RPar,
    LPar,
    Add,
    Sub,
    Mul,
    Div,
    Pow
}

impl TryFrom<char> for Operator {
    type Error = char;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Self::LPar),
            ')' => Ok(Self::RPar),
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Sub),
            '*' => Ok(Self::Mul),
            '/' => Ok(Self::Div),
            '^' => Ok(Self::Pow),
            _ => return Err(value),
        }
    }
}

impl Operator {
    fn priority(self) -> u8 {
        match self {
            Self::RPar | Self::LPar => 0,
            Self::Add | Self::Sub => 1,
            Self::Mul | Self::Div => 2,
            Self::Pow => 3,
        }
    }
}

struct Stack(Vec<Operator>);

impl Stack {
    fn push(&mut self, el: Operator) {
        self.0.push(el);
    }

    fn top(&self) -> Operator {
        *self.0.last().unwrap_or(&Operator::LPar)
    }

    fn pop(&mut self) -> Operator {
        self.0.pop().unwrap()
    }
}

fn eval(tokens: Vec<Token>) -> f64 {
    let mut stack = Vec::new();

    for token in tokens {
        match token {
            Token::Number(num) => stack.push(num),
            Token::Operator(op) => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();

                match op {
                    Operator::Add => stack.push(x + y),
                    Operator::Mul => stack.push(x * y),
                    Operator::Pow => stack.push(x.powf(y)),
                    Operator::Div => stack.push(y / x),
                    Operator::Sub => stack.push(y - x),
                    _ => unreachable!()
                }
            }
        }
    }

    stack[0]
}

#[tauri::command]
fn parse_and_eval(inp: String) -> Result<f64, String> {
    let mut stack = Stack(Vec::new());
    let mut postfix = Vec::new();

    let mut cur_num = String::new();

    for i in inp.trim().chars() {
        match Operator::try_from(i) {
            Ok(op) => {
                if !cur_num.is_empty() {
                    postfix.push(Token::Number(cur_num.parse().map_err(|err: ParseFloatError| err.to_string())?));
                    cur_num = String::new();
                }

                match op {
                    Operator::LPar => stack.push(op),

                    Operator::RPar => {
                        while stack.top() != Operator::LPar {
                            postfix.push(Token::Operator(stack.pop()))
                        };
                        stack.pop();
                    }

                    Operator::Add | Operator::Sub | Operator::Mul | Operator::Div | Operator::Pow => {
                        while op.priority() < stack.top().priority() {
                            postfix.push(Token::Operator(stack.pop()))
                        };
                        stack.push(op);
                    }
                }
            }

            Err(digit) => cur_num.push(digit)
        }
    }

    if !cur_num.is_empty() {
        postfix.push(Token::Number(cur_num.parse().map_err(|err: ParseFloatError| err.to_string())?))
    }

    while !stack.0.is_empty() {
        postfix.push(Token::Operator(stack.pop()));
    }

    Ok(eval(postfix))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse_and_eval])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
