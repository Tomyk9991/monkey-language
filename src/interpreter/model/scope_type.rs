#[derive(Debug)]
pub enum ScopeType {
    Fn,
    If
}

pub struct ScopeSplitterIterator {
    current: usize
}

pub struct ScopeTypeIterator {
    current: usize
}

impl ScopeSplitterIterator {
    pub fn new() -> Self {
        Self { current: 0 }
    }
}

impl ScopeTypeIterator {
    pub fn new() -> Self {
        Self { current: 0 }
    }
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
                Some((vec![r"if\s*\(.*?\)\s*\{", r"if\s*\(.*?\)\s*\{.*?\}",], ScopeType::If))
            },
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
            _ => None
        }
    }
}