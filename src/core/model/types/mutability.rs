#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Mutability {
    Mutable,
    Immutable,
}

impl From<bool> for Mutability {
    fn from(value: bool) -> Self {
        if value {
            Mutability::Mutable
        } else {
            Mutability::Immutable
        }
    }
}

impl From<&str> for Mutability {
    fn from(s: &str) -> Self {
        match s {
            "mut" => Mutability::Mutable,
            _ => Mutability::Immutable
        }
    }
}
