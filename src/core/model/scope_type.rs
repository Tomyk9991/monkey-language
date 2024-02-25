
/// ScopeType is an enum that represents the different types of scopes that can be found in the monkey language.
#[derive(Debug)]
pub enum ScopeType {
    Fn,
    If,
    Else,
    For,
}

/// `ScopeSplitterIterator` is an iterator that returns a tuple of a vector of regexes and a ScopeType.
#[derive(Default)]
pub struct ScopeSplitterIterator {
    current: usize
}

/// `ScopeTypeIterator` is an iterator that returns a tuple of a string and a ScopeType.
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
                Some((vec![
                    r"fn .*?\(.*?\):\s.*?\{",
                    r"fn .*?\(.*?\)\s\{ ",
                    r"fn .*?\(.*?\)\s\{.*?\}",
                    r"fn .*?\(.*?\):\s.*?\{.*?\}",

                    r"(extern )fn .*?\(.*?\):\s.*?\;",
                ], ScopeType::Fn))
            },
            1 => {
                self.current += 1;
                Some((vec![
                    r"if\s*\(.*?\)\s*\{",
                    r"if\s*\(.*?\)\s*\{.*?\}"
                ], ScopeType::If))
            },
            2 => {
                self.current += 1;
                Some((vec![
                    r"else\s*\{",
                    r"else\s*\{.*?\}",
                    r"\s*\}\s*else\s*\{"
                ], ScopeType::Else))
            },
            3 => {
                self.current += 1;
                Some((vec![
                    r"for\s*\(.*?\)\s*\{",
                    r"for\s*\(.*?\)\s*\{.*?\}",
                ], ScopeType::Fn))
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
            },
            3 => {
                self.current += 1;
                Some(("for ", ScopeType::For))
            }
            _ => None
        }
    }
}