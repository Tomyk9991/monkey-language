use std::fmt::{Display, Formatter};

#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub struct IntegerAST {
    // Must be stored as a string literal, because
    // you can have a bigger value than an i64. consider every number that's between i64::MAX and u64::MAX
    pub value: String,
    pub ty: IntegerType
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub enum IntegerType {
    I8,
    U8,
    I16,
    U16,
    #[default]
    I32,
    U32,
    I64,
    U64,
}

impl Display for IntegerAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
