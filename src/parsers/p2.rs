use super::ast::{Variables, AST};
use crate::{parsers::p3::p3, tokens::Token};
use colored::*;

/// List of reserved words...
///
/// This list is currently not exhaustive and only contains the words that are
/// already used in the language.
pub(crate) static RESERVED_WORDS: &[&str] = &["put", "spread_vars"];

fn error_msg(line: usize, msg: &str) -> String {
    format!("{}: {}", format!("Line {}", line).red().bold(), msg)
}

pub fn p2(
    prevtok: &Token,
    iter: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    line: &mut usize,
) -> Result<Vec<AST>, Vec<String>> {
    let mut ast = Vec::new();
    let mut errors = Vec::new();

    if let Token::Iden(type_token) = prevtok {
        let type_str: String = type_token.letters.iter().collect();
        let base_var = match type_str.as_str() {
            "i8" => Variables::I8(0),
            "i16" => Variables::I16(0),
            "i32" => Variables::I32(0),
            "i64" => Variables::I64(0),
            "i128" => Variables::I128(0),
            "u8" => Variables::U8(0),
            "f32" => Variables::F32(0.0),
            "f64" => Variables::F64(0.0),
            "char" => Variables::Char(' '),
            "str" => Variables::String(String::new()),
            _ => {
                errors.push(error_msg(*line, &format!("Unknown variable type: {}", type_str)));
                Variables::I8(0)
            }
        };

        let name = match iter.next() {
            Some(Token::Iden(name_token)) => name_token.letters.iter().collect::<String>(),
            _ => {
                errors.push(error_msg(*line, "Expected variable name after type"));
                String::new()
            }
        };

        if RESERVED_WORDS.contains(&name.as_str()) {
            errors.push(error_msg(*line, &format!("Variable name '{}' is reserved", name)));
        }

        if let Some(Token::EqSign) = iter.next() {
            let value_str = match iter.next() {
                Some(Token::Iden(value_token)) => value_token.letters.iter().collect::<String>(),
                _ => {
                    errors.push(error_msg(*line, "Expected variable value after '='"));
                    String::new()
                }
            };

            let var_value = match base_var {
                Variables::I8(_) if value_str.parse::<i8>().is_ok() => Variables::I8(value_str.parse().unwrap()),
                Variables::I16(_) if value_str.parse::<i16>().is_ok() => Variables::I16(value_str.parse().unwrap()),
                Variables::I32(_) if value_str.parse::<i32>().is_ok() => Variables::I32(value_str.parse().unwrap()),
                Variables::I64(_) if value_str.parse::<i64>().is_ok() => Variables::I64(value_str.parse().unwrap()),
                Variables::I128(_) if value_str.parse::<i128>().is_ok() => Variables::I128(value_str.parse().unwrap()),
                Variables::U8(_) if value_str.parse::<u8>().is_ok() => Variables::U8(value_str.parse().unwrap()),
                Variables::F32(_) if value_str.parse::<f32>().is_ok() => Variables::F32(value_str.parse().unwrap()),
                Variables::F64(_) if value_str.parse::<f64>().is_ok() => Variables::F64(value_str.parse().unwrap()),
                Variables::Char(_) if value_str.len() == 1 => Variables::Char(value_str.chars().next().unwrap()),
                Variables::String(_) => Variables::String(value_str.clone()),
                _ => {
                    errors.push(error_msg(*line, &format!("Invalid literal value for variable '{}': {}", name, value_str)));
                    base_var
                }
            };

            ast.push(AST::VarDecl(name.clone(), var_value));
            return Ok(ast);
        } else {
            errors.push(error_msg(*line, "Expected '=' after variable name"));
        }
    }

    while let Some(token) = iter.next() {
        match token {
            Token::Iden(_) => {
                let sub_result = p3(token, iter, line);
                match sub_result {
                    Ok(mut sub_ast) => ast.append(&mut sub_ast),
                    Err(sub_errors) => errors.extend(sub_errors),
                }
            }
            Token::AtTheRate => match p3(prevtok, iter, line) {
                Ok(mut gast) => ast.append(&mut gast),
                Err(gerrors) => errors.extend(gerrors),
            },
            _ => errors.push(error_msg(*line, &format!("Unexpected token: {:#?}", token))),
        }
    }

    if errors.is_empty() {
        Ok(ast)
    } else {
        Err(errors)
    }
}
