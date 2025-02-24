use checker::checkast;
use colored::Colorize;

use crate::tokens::Token;

pub mod p1;
pub mod p2;
pub mod p3;
pub mod builtins;
pub mod asmmaker;
pub mod ast;
pub mod checker;
pub fn parse(tokens : &Vec<Token>) -> Vec<ast::AST> {
    match p1::p1(tokens){
        Ok(ast) => {
            if checkast(&ast, false){
                println!("{}","Error: Invalid AST".red());
                std::process::exit(1);
            }
            else{
                ast
            }
        }
        Err(errs) => {
            if !errs.is_empty() {
                println!("┌[Parser Error Messages]");
                for err in errs {
                    println!("├─ {}", err);
                }
            }
            
            std::process::exit(1);
        }
    }
}