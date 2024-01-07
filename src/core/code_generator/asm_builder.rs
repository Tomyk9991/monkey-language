use std::fmt::Display;

/// A utility struct for construction asm related strings
pub struct ASMBuilder;

impl ASMBuilder {
    pub fn push(argument: &str) -> String {
        argument.to_string()
    }

    pub fn comment_line(argument: &str) -> String {
        format!("; {}\n", argument)
    }

    pub fn line(argument: &str) -> String {
        format!("{}{}\n", " ".repeat(0), argument)
    }

    pub fn ident_line(argument: &str) -> String {
        format!("{}{}\n", " ".repeat(4), argument)
    }

    pub fn mov_ident_line<T: Display, P: Display>(destination: T, source: P) -> String {
        let source = source.to_string();
        let destination = destination.to_string();

        if source == destination {
            return String::new();
        }

        if source.ends_with('\n') {
            format!("{}mov {}", " ".repeat(4), ASMBuilder::mov(&destination, &source))
        } else {
            format!("{}mov {}\n", " ".repeat(4), ASMBuilder::mov(&destination, &source))
        }
    }

    pub fn mov(destination: &str, source: &str) -> String {
        format!("{}, {}", destination, source)
    }

    #[allow(unused)]
    pub fn ident_comment_line(comment: &str) -> String {
        format!("{}; {}\n", " ".repeat(4), comment)
    }

    pub fn ident(argument: &str) -> String {
        format!("{}{}", " ".repeat(4), argument)
    }
}