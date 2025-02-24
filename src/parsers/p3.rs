use crate::tokens::Token;
use super::{ast::AST, builtins::puts};

pub fn p3(
    _prevtok: &Token,
    iter: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    line: &mut usize,
) -> Result<Vec<AST>, Vec<String>> {
    let mut ast: Vec<AST> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    if let Some(Token::Iden(n)) = iter.next() {
        let callname = n.letters.iter().collect::<String>();
        match callname.as_str() {
            "put" => {
                if iter.peek() == Some(&&Token::LSmallB) {
                    puts::process_puts(iter, &mut errors, line, &mut ast);
                } else {
                    errors.push(format!("Line {}: Expected ( after put", line));
                }
            }
            _ => {
                errors.push(format!("Line {}: Unknown directive @{}", line, callname));
            }
        }
    } else {
        errors.push(format!("Line {}: Expected identifier after @", line));
    }

    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(ast)
    }
}
