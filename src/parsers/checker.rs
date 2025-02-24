use super::ast;

#[allow(unused)]
pub fn checkast(ast: &Vec<ast::AST>,fromfn : bool) -> bool {
    let mut declfns : Vec<String> = Vec::new();
    let mut declvars : Vec<String> = Vec::new();
    let mut errsfnd = false;
    for node in ast {
        match node {
            ast::AST::Fn(name,args ,body ) => {
                if fromfn {
                    eprintln!("invalid fn declared inside a function");
                    errsfnd = true;
                }
                if declfns.contains(&name.to_string()) {
                    eprintln!("function {} already declared",name);
                    errsfnd = true;
                }
                declfns.push(name.to_string());
                checkast(body, true);
            }
            ast::AST::VarDecl(name, vtype) => {
                if !fromfn {
                    eprintln!("invalid variable declared outside a function");
                    errsfnd = true;
                }
                if declvars.contains(&name.to_string()) {
                    eprintln!("variable {} already declared",name);
                    errsfnd = true;
                }
                declvars.push(name.to_string());
            }
            ast::AST::Put(fd, text) => {
                if !fromfn {
                    eprintln!("invalid put statement declared outside a function");
                    errsfnd = true;
                }
            }
            _ => {}
        }
    }
    if declfns.contains(&"main".to_string()) {
        errsfnd = false;
    }
    else{
        eprintln!("main function not declared");
        errsfnd = true;
    }
    errsfnd
}