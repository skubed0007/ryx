#[derive(Debug, Clone)]
pub enum AST{
    Fn(String,Vec<(String,Variables)>,Vec<AST>),
    VarDecl(String,Variables),
    Put(i8,String),
}

#[derive(Debug, Clone)]
pub enum Variables{
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    F32(f32),
    F64(f64),
    Char(char),
    String(String),
}