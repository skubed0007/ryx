use std::{env::args, fs::{metadata, File}, io::Read, path::Path, process::exit};

use colored::Colorize;
use parsers::{asmmaker::mk_asm, parse};
use tokens::Token;
use xasm_rs::mkwnasm;
pub mod parsers;
pub mod tokens;

fn main(){
    let args = args().collect::<Vec<String>>();
    let cmd = &args[1];
    let proj = &args[2];
    let _args= &args[3..];
    match cmd.as_str() {
        "build" => {
            println!("{}{}...","Building project ".bold().green() ,proj.green());
            let projp = Path::new(proj);
            if metadata(projp).is_ok(){
                if metadata(projp).unwrap().is_dir(){
                    eprintln!("{}","Expected a file".red());
                    exit(1);
                }
                let mut code = String::new();
                let mut projf = File::open(projp).unwrap();
                match projf.read_to_string(&mut code){
                    Ok(_) => {
                        println!("lexing...");
                        let tokens = Token::run_lexical_analyzer(&code);
                        println!("generating ast....");
                        let ast = parse(&tokens);
                        println!("making asm code...");
                        let asm = mk_asm(&ast);
                        println!("compiling asm code...");
                        if let Err(shall_not_happen_if_followed_docs) = mkwnasm::compile_with_nasm(&asm,xasm_rs::osconfig::OsConfig::Linux_X86_64){
                            eprintln!("{}{}","failed to compile with nasm\nnexact error ~ {}".red(),shall_not_happen_if_followed_docs);
                        }
                        println!("finished! find the binary at \"./out\"");
                        
                    }
                    Err(e) => {
                        eprintln!("{}{}","Error reading file\nerr ~ ".red(),e);
                        exit(1);
                    }
                }
                
            }
        }
        _ => {}
    }
}