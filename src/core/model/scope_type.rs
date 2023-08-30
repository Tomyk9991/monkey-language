#[derive(Debug)]
pub enum ScopeType {
    Fn,
    If,
    Else,
}

#[derive(Default)]
pub struct ScopeSplitterIterator {
    current: usize
}

#[derive(Default)]
pub struct ScopeTypeIterator {
    current: usize
}


impl Iterator for ScopeSplitterIterator {
    type Item = (Vec<&'static str>, ScopeType);

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            0 => {
                self.current += 1;
                Some((vec![r"fn .*?\(.*?\):\s.*?\{", r"fn .*?\(.*?\):\s.*?\{.*?\}"], ScopeType::Fn))
            },
            1 => {
                self.current += 1;
                Some((vec![r"if\s*\(.*?\)\s*\{", r"if\s*\(.*?\)\s*\{.*?\}"], ScopeType::If))
            },
            2 => {
                self.current += 1;
                Some((vec![r"else\s*\{", r"else\s*\{.*?\}", r"\s*\}\s*else\s*\{"], ScopeType::Else))
            }
            _ => None
        }
    }
}

impl Iterator for ScopeTypeIterator {
    type Item = (&'static str, ScopeType);

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            0 => {
                self.current += 1;
                Some(("fn ", ScopeType::Fn))
            },
            1 => {
                self.current += 1;
                Some(("if ", ScopeType::If))
            },
            2 => {
                self.current += 1;
                Some(("else", ScopeType::Else))
            }
            _ => None
        }
    }
}