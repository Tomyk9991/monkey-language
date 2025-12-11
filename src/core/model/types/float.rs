use std::fmt::{Display, Formatter};

#[derive(Debug, PartialOrd, PartialEq, Clone, Default)]
pub struct FloatAST {
    // https://pastebin.com/DWcHQbT5
    // there is no need to use a string literal instead of a f64 like in the integerASTNode, because
    // you cant have a float that's bigger than the biggest value of f64. but you can have a bigger value than a i64. consider every number that's between i64::MAX and u64::MAX
    pub value: f64,
    pub ty: FloatType,
}

#[derive(Debug, Default, PartialOrd, PartialEq, Eq, Hash, Clone)]
pub enum FloatType {
    #[default]
    Float32,
    Float64,
}


impl Display for FloatAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
