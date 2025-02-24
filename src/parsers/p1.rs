use super::{
    ast::{Variables, AST},
    p2::p2,
};
use crate::tokens::Token;

#[derive(PartialEq, Debug)]
pub enum ExpectedToken {
    Name,
    LParen,
    LBrace,
    RBrace,
    RParen,
    EqSign,
}

pub fn p1(tokens: &[Token]) -> Result<Vec<AST>, Vec<String>> {
    let mut line: usize = 1;
    let mut ast = Vec::new();
    let mut errors = Vec::new();
    let mut tok_iter = tokens.iter().peekable();

    while let Some(token) = tok_iter.next() {
        if let Token::EOL = token {
            line += 1;
            continue;
        }
        match token {
            Token::FnK => match parse_function(&mut tok_iter, &mut line) {
                Ok(func) => {
                    ast.push(func);
                }
                Err(mut errs) => {
                    errors.append(&mut errs);
                }
            },
            Token::EOF => break,
            _ => match p2(token, &mut tok_iter, &mut line) {
                Ok(mut res) => {
                    ast.append(&mut res);
                }
                Err(mut errs) => {
                    errors.append(&mut errs);
                }
            },
        }
    }

    if errors.is_empty() {
        Ok(ast)
    } else {
        Err(errors)
    }
}

fn parse_function<'a, I>(tok_iter: &mut I, line: &mut usize) -> Result<AST, Vec<String>>
where
    I: Iterator<Item = &'a Token>,
{
    let mut errors = Vec::new();
    let mut name = String::new();
    let mut collected_args = String::new();
    let mut collected_body = Vec::new();
    let mut expected = ExpectedToken::Name;

    while let Some(token) = tok_iter.next() {
        if let Token::EOL = token {
            *line += 1;
            continue;
        }
        match (token, &expected) {
            (Token::Iden(n), ExpectedToken::Name) => {
                name = n.letters.iter().collect();
                expected = ExpectedToken::LParen;
            }
            (Token::LSmallB, ExpectedToken::LParen) => {
                collected_args = parse_arguments(tok_iter, line);
                expected = ExpectedToken::LBrace;
            }
            (Token::LCurlyB, ExpectedToken::LBrace) => {
                collected_body = parse_body(tok_iter, line);
                break;
            }
            (t, _) => {
                let err = format!(
                    "Line {}: Unexpected token {:?} in function declaration",
                    line, t
                );
                errors.push(err);
                break;
            }
        }
    }

    let args = parse_args_string(&collected_args, line);
    let body_ast = match p1(&collected_body) {
        Ok(a) => a,
        Err(mut errs) => {
            errors.append(&mut errs);
            Vec::new()
        }
    };

    if errors.is_empty() {
        Ok(AST::Fn(name, args, body_ast))
    } else {
        Err(errors)
    }
}

fn parse_arguments<'a, I>(tok_iter: &mut I, line: &mut usize) -> String
where
    I: Iterator<Item = &'a Token>,
{
    let mut arg_str = String::new();
    let mut paren_depth = 1;

    while let Some(token) = tok_iter.next() {
        if let Token::EOL = token {
            *line += 1;
            continue;
        }
        match token {
            Token::LSmallB => paren_depth += 1,
            Token::RSmallB => {
                paren_depth -= 1;
                if paren_depth == 0 {
                    break;
                }
            }
            Token::Iden(n) => {
                let arg = n.letters.iter().collect::<String>();
                arg_str.push_str(&arg);
            }
            _ => {}
        }
    }

    arg_str
}

fn parse_body<'a, I>(tok_iter: &mut I, line: &mut usize) -> Vec<Token>
where
    I: Iterator<Item = &'a Token>,
{
    let mut body_tokens = Vec::new();
    let mut brace_depth = 1;

    while let Some(token) = tok_iter.next() {
        if let Token::EOL = token {
            *line += 1;
            continue;
        }
        match token {
            Token::LCurlyB => {
                brace_depth += 1;
                body_tokens.push(token.clone());
            }
            Token::RCurlyB => {
                brace_depth -= 1;
                if brace_depth == 0 {
                    break;
                }
                body_tokens.push(token.clone());
            }
            _ => body_tokens.push(token.clone()),
        }
    }

    body_tokens
}

fn parse_args_string(args_str: &str, _line: &usize) -> Vec<(String, Variables)> {
    let mut args = Vec::new();
    if args_str.trim().is_empty() {
        return args;
    }
    for arg in args_str.split(',') {
        let trimmed = arg.trim();
        if trimmed.is_empty() {
            continue;
        }
        let parts: Vec<&str> = trimmed.split(':').collect();
        if parts.len() != 2 {
            continue;
        }
        let arg_name = parts[0].trim().to_string();
        let arg_type = match parts[1].trim() {
            "i8" => Variables::I8(0),
            "i16" => Variables::I16(0),
            _ => continue,
        };
        args.push((arg_name, arg_type));
    }
    args
}
