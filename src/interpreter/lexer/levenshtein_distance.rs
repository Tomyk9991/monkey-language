use crate::interpreter::io::code_line::CodeLine;

static IGNORED_LEVENSHTEIN_CASE: &str = "LEVENSHTEIN_IGNORE";

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
    fn distance<P: Into<String>, K: Into<String>>(a: P, b: K) -> usize {
        let string_a = a.into();
        let string_b = b.into();

        levenshtein_distance(&string_a, &string_b)
    }

    fn distance_from_code_line(code_line: &CodeLine) -> usize;
}

pub trait SegmentTransform {
    fn transform(&self, values: &mut Vec<String>);
}

pub struct QuoteSummarizeTransform;
pub struct MethodCallSummarizeTransform;
pub struct EmptyParenthesesExpand;
pub struct ArgumentsIgnoreSummarizeTransform;


impl SegmentTransform for QuoteSummarizeTransform {
    /// Implementing SegmentTransform for QuoteSummarizeTransform.
    /// This struct will handle the transformation of quoted segments within an input vector of strings.
    /// The `transform` method takes a mutable reference to a vector of strings and modifies it by:
    ///   1. Joining the quoted segments into a single string value.
    ///   2. Replacing the original segmented values in the vector with the joined string value.
    /// The main goal of this implementation is to handle and merge the provided quoted segments in the input vector of strings.
    fn transform(&self, values: &mut Vec<String>) {
        let mut new_vec = vec![];

        let mut start_i = 0;
        let mut collecting = false;

        let length = values.len();

        for i in 0..length {
            let value = &values[i];

            if value.starts_with('\"') {
                start_i = i;
                collecting = true;
            }

            if value.ends_with('\"') {
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

impl SegmentTransform for MethodCallSummarizeTransform {
    /// Implementing SegmentTransform for MethodCallSummarizeTransform.
    /// This struct handles the transformation of method calls within an input vector of strings.
    /// The `transform` method takes a mutable reference to a Vec<String> and modifies it by:
    ///   1. Joining the method call segments (including its arguments) into a single string value.
    ///   2. Replacing the original segmented values in the vector with the joined string value.
    /// The primary goal is to handle and merge method call segments in the provided Vec<String>.
    fn transform(&self, values: &mut Vec<String>) {
        let mut new_vec = vec![];

        let mut start_i = 0;
        let mut collecting = false;

        let length = values.len();

        for i in 0..length {
            let value = &values[i];

            if value == "(" {
                start_i = i - 1;
                collecting = true;
            }

            if value == ")" {
                collecting = false;
                new_vec.pop();
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


impl SegmentTransform for EmptyParenthesesExpand {
    /// Implementing SegmentTransform for EmptyParenthesesExpand.
    /// This struct focuses on transforming input vector of strings containing empty parentheses.
    /// Thse `transform` method takes a mutable reference to a Vec<String> and modifies it by:
    ///   1. Adding IGNORED_LEVENSHTEIN_CASE in between empty parentheses.
    ///   2. Replacing the original segmented values with modified values in the vector.
    /// The purpose of this implementation is to handle empty method calls within the input vector of strings.
    fn transform(&self, values: &mut Vec<String>) {
        let mut new_vec: Vec<String> = vec![];
        let length = values.len();

        let mut i: usize = 0;
        while i < length {
            let value = &values[i];

            if value == "(" {
                if let Some(next_value) = values.get(i + 1) {
                    if next_value == ")" {
                        // this is an empty method-call
                        // transform it to:
                        // ( IGNORE )
                        new_vec.push("(".to_string());
                        new_vec.push(IGNORED_LEVENSHTEIN_CASE.to_string());
                        new_vec.push(")".to_string());

                        i += 2;
                        continue;
                    }
                }
            }

            new_vec.push(value.to_string());

            i += 1;
        }


        *values = new_vec;
    }
}

impl SegmentTransform for ArgumentsIgnoreSummarizeTransform {
    /// Implementing SegmentTransform for ArgumentsIgnoreSummarizeTransform.
    /// This implementation is responsible for transforming input vector of strings containing parenthesized arguments.
    /// The `transform` method takes a mutable reference to a Vec<String> and modifies it by:
    ///   1. Replacing the parenthesized segments with "( IGNORE )".
    /// It effectively ignores method call arguments present in between parentheses within the input vector of strings.
    fn transform(&self, values: &mut Vec<String>) {
        let mut new_vec: Vec<String> = vec![];
        let length = values.len();

        let mut i: usize = 0;

        let mut open = false;
        let mut last_bracket = None;
        while i < length {
            let value = &values[i];

            if value == "(" && !open {
                // transform it to:
                // ( IGNORE )
                open = true;
                if let Some(index) = values.iter().rposition(|r| r == ")") {
                    last_bracket = Some(index);

                    new_vec.push("(".to_string());
                    new_vec.push(IGNORED_LEVENSHTEIN_CASE.to_string());
                    new_vec.push(")".to_string());
                }

                continue;
            }

            if let Some(last_bracket) = last_bracket {
                if last_bracket == i {
                    open = false;
                }
            }

            if !open {
                new_vec.push(value.to_string());
            }

            i += 1;
        }


        *values = new_vec;
    }
}


#[derive(Default)]
pub struct PatternedLevenshteinString {
    data: Vec<String>,
}

impl From<PatternedLevenshteinString> for String {
    fn from(value: PatternedLevenshteinString) -> Self {
        value.data.join(" ")
    }
}

impl PatternedLevenshteinString {
    pub fn match_to(value: &str, pattern: &PatternedLevenshteinString, summarize: Vec<Box<dyn SegmentTransform>>) -> String {
        let mut segments = value.split_whitespace()
            .map(|a| a.to_string())
            .collect::<Vec<_>>();

        for summ in summarize {
            summ.transform(&mut segments);
        }

        for (i, datum) in pattern.data.iter().enumerate() {
            if datum == PatternedLevenshteinString::ignore() {
                if i >= segments.len() { break; }
                segments[i] = PatternedLevenshteinString::ignore().to_string();
            }
        }

        segments.join(" ")
    }

    pub fn insert(mut self, value: &str) -> Self {
        self.data.push(value.to_string());
        self
    }

    pub fn ignore() -> &'static str { IGNORED_LEVENSHTEIN_CASE }
}