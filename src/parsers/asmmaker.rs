use xasm_rs::{
    genasm::genasm,
    init::{self, Func},
};

use super::ast::{self, Variables};

pub fn mk_asm(ast: &Vec<ast::AST>) -> String {
    let mut xasm = init::Xasm::new();
    for node in ast {
        match node {
            ast::AST::Fn(name, _args, body) => {
                if name == "main" {
                    for node in body {
                        match node {
                            ast::AST::VarDecl(name, vtype) => match vtype {
                                Variables::Char(c) => {
                                    xasm.vars.push(init::Vars::Char(name.clone(), *c));
                                }
                                Variables::I8(i) => {
                                    xasm.vars.push(init::Vars::I8(name.clone(), *i));
                                }
                                Variables::I16(i) => {
                                    xasm.vars.push(init::Vars::I16(name.clone(), *i));
                                }
                                Variables::I32(i) => {
                                    xasm.vars.push(init::Vars::I32(name.clone(), *i));
                                }
                                Variables::I64(i) => {
                                    xasm.vars.push(init::Vars::I64(name.clone(), *i));
                                }
                                Variables::I128(i) => {
                                    xasm.vars.push(init::Vars::I64(name.clone(), *i as i64));
                                }
                                Variables::F32(f) => {
                                    xasm.vars.push(init::Vars::F32(name.clone(), *f));
                                }
                                Variables::F64(f) => {
                                    xasm.vars.push(init::Vars::F64(name.clone(), *f));
                                }
                                Variables::U8(i) => {
                                    xasm.vars.push(init::Vars::I8(name.clone(), *i as i8));
                                }
                                Variables::String(s) => {
                                    xasm.vars.push(init::Vars::String(name.clone(), s.clone()));
                                }
                            },
                            ast::AST::Put(fd, s) => {
                                xasm.tokens.push(init::Tokens::print(
                                    match fd {
                                        1 => init::FileDescriptor::STDOUT,
                                        2 => init::FileDescriptor::STDERR,
                                        0 => init::FileDescriptor::STDIN,
                                        _ => init::FileDescriptor::STDOUT,
                                    },
                                    s.clone().chars().collect(),
                                ));
                            }
                            _ => {}
                        }
                    }
                } else {
                    xasm.funcs.push(Func {
                        name: name.clone(),
                        args: {
                            //match variables and return var
                            None
                        },
                        mut_args : {
                            //match and return
                            None
                        },
                        ret : None,
                        body: {
                            let mut xasm = init::Xasm::new();
                            for node in body {
                                match node {
                                    ast::AST::VarDecl(name, vtype) => match vtype {
                                        Variables::Char(c) => {
                                            xasm.vars.push(init::Vars::Char(name.clone(), *c));
                                        }
                                        Variables::I8(i) => {
                                            xasm.vars.push(init::Vars::I8(name.clone(), *i));
                                        }
                                        Variables::I16(i) => {
                                            xasm.vars.push(init::Vars::I16(name.clone(), *i));
                                        }
                                        Variables::I32(i) => {
                                            xasm.vars.push(init::Vars::I32(name.clone(), *i));
                                        }
                                        Variables::I64(i) => {
                                            xasm.vars.push(init::Vars::I64(name.clone(), *i));
                                        }
                                        Variables::I128(i) => {
                                            xasm.vars
                                                .push(init::Vars::I64(name.clone(), *i as i64));
                                        }
                                        Variables::F32(f) => {
                                            xasm.vars.push(init::Vars::F32(name.clone(), *f));
                                        }
                                        Variables::F64(f) => {
                                            xasm.vars.push(init::Vars::F64(name.clone(), *f));
                                        }
                                        Variables::U8(i) => {
                                            xasm.vars.push(init::Vars::I8(name.clone(), *i as i8));
                                        }
                                        Variables::String(s) => {
                                            xasm.vars
                                                .push(init::Vars::String(name.clone(), s.clone()));
                                        }
                                    },
                                    ast::AST::Put(fd, s) => {
                                        xasm.tokens.push(init::Tokens::print(
                                            match fd {
                                                1 => init::FileDescriptor::STDOUT,
                                                2 => init::FileDescriptor::STDERR,
                                                0 => init::FileDescriptor::STDIN,
                                                _ => init::FileDescriptor::STDOUT,
                                            },
                                            s.clone().chars().collect(),
                                        ));
                                    }
                                    _ => {}
                                }
                            }
                            xasm
                        },
                    });
                }
            }
            _ => {}
        }
    }
    let asm = genasm(&xasm, &xasm_rs::osconfig::OsConfig::Linux_X86_64);
    asm
}
