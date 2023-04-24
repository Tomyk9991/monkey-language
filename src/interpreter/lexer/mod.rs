use crate::interpreter::io::code_line::CodeLine;

pub mod lexer;
pub mod scope;
pub mod token;
pub mod tokens;

static IGNORED_LEVENSHTEIN_CASE: &'static str = "LEVENSHTEIN_IGNORE";

pub trait TryParse {
    type Output;

    fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self::Output>;
}

pub mod levenshtein_distance {
    use crate::interpreter::lexer::IGNORED_LEVENSHTEIN_CASE;

    // https://i.imgur.com/AI56ag7.png
    ///```rust
    /// let value1 = "kitten".to_string();
    /// let value2 = "sitting".to_string();
    /// assert_eq!(3, monkey_language::interpreter::lexer::levenshtein_distance::levenshtein_distance(value1.as_str(), value2.as_str()));
    /// ```
    pub fn levenshtein_distance(a: &str, b: &str) -> usize {
        let mut result = 0;

        if a == b {
            return result;
        }

        let length_a = a.chars().count();
        let length_b = b.chars().count();

        if length_a == 0 {
            return length_b;
        }

        if length_b == 0 {
            return length_a;
        }

        let mut cache: Vec<usize> = (1..).take(length_a).collect();
        let mut distance_a;
        let mut distance_b;

        for (index_b, code_b) in b.chars().enumerate() {
            result = index_b;
            distance_a = index_b;

            for (index_a, code_a) in a.chars().enumerate() {
                distance_b = if code_a == code_b {
                    distance_a
                } else {
                    distance_a + 1
                };

                distance_a = cache[index_a];

                result = if distance_a > result {
                    if distance_b > result {
                        result + 1
                    } else {
                        distance_b
                    }
                } else if distance_b > distance_a {
                    distance_a + 1
                } else {
                    distance_b
                };

                cache[index_a] = result;
            }
        }

        result
    }

    pub trait PatternedLevenshteinDistance {
        fn distance<P: Into<String>, K: Into<String>>(a: P, b: K) -> usize;
    }

    pub struct PatternedLevenshteinString {
        data: Vec<String>,
    }

    impl From<PatternedLevenshteinString> for String {
        fn from(value: PatternedLevenshteinString) -> Self {
            value.data.join(" ")
        }
    }

    impl Default for PatternedLevenshteinString {
        fn default() -> Self {
            Self {
                data: vec![],
            }
        }
    }

    impl PatternedLevenshteinString {
        pub fn match_to(value: &str, pattern: &PatternedLevenshteinString) -> String {
            let mut segments = value.split_whitespace()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>();
            
            normalize_quotes(&mut segments);
            
            for (i, datum) in pattern.data.iter().enumerate() {
                if datum == PatternedLevenshteinString::ignore() {
                    segments[i] = PatternedLevenshteinString::ignore().to_string();
                }
            }

            return segments.join(" ");
        }

        pub fn insert(mut self, value: &str) -> Self {
            self.data.push(value.to_string());
            self
        }
        
        pub fn ignore() -> &'static str { IGNORED_LEVENSHTEIN_CASE }
    }

    fn normalize_quotes(values: &mut Vec<String>) {
        let mut new_vec = vec![];

        let mut start_i = 0;
        let mut collecting = false;

        let length = values.len();
        
        for i in 0..length {
            let value = &values[i];

            if value.starts_with("\"") {
                start_i = i;
                collecting = true;
            }
            
            if value.ends_with("\"") {
                collecting = false;
                new_vec.push(values[start_i..i + 1].join(" ").clone());
                continue;
            }

            if !collecting {
                new_vec.push(value.to_string());
            }
            
        }
        
        
        *values = new_vec;
    }
}