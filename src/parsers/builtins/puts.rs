use crate::{
    parsers::ast::{self, AST},
    tokens::Token,
};

#[allow(unused)]
pub fn process_puts(
    iter: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    errors: &mut Vec<String>,
    line: &mut usize,
    ast: &mut Vec<ast::AST>,
) {
    let mut fd = 1; // Default to stdout
    let mut message = String::new();
    let mut in_string = false;

    // Expect opening parenthesis
    if let Some(Token::LSmallB) = iter.next() {
        // Parse file descriptor
        match iter.next() {
            Some(Token::Iden(fd_token)) => {
                let fd_str: String = fd_token.letters.iter().collect();
                fd = match fd_str.to_lowercase().as_str() {
                    "stdin" | "0" => 0,
                    "stdout" | "1" => 1,
                    "stderr" | "2" => 2,
                    _ => {
                        errors.push(format!("Line {}: Invalid file descriptor: {}", line, fd_str));
                        1
                    }
                };
            }
            _ => {
                errors.push(format!("Line {}: Expected file descriptor after (", line));
                return;
            }
        }
        // Expect comma after FD
        if let Some(Token::Comma) = iter.next() {
            // Parse message
            while let Some(tok) = iter.next() {
                match tok {
                    Token::Quote => {
                        in_string = !in_string;
                        if !in_string {
                            break;
                        }
                    }
                    Token::Iden(word) if in_string => {
                        message.push(' ');
                        message.push_str(&word.letters.iter().collect::<String>());
                    }
                    Token::RSmallB if !in_string => break,
                    _ => {
                        if in_string {
                            errors.push(format!("Line {}: Unexpected token in string: {:?}", line, tok));
                        }
                    }
                }
            }
        } else {
            errors.push(format!("Line {}: Expected comma after file descriptor", line));
        }
    } else {
        errors.push(format!("Line {}: Expected ( after put", line));
        return;
    }

    // Expect closing parenthesis and semicolon
    if iter.next() != Some(&Token::RSmallB) {
        errors.push(format!("Line {}: Expected )", line));
    }
    if iter.next() != Some(&Token::SemiC) {
        errors.push(format!("Line {}: Expected ;", line));
    }
    ast.push(AST::Put(fd, message.trim().to_string()));
}
