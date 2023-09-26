/// A utility struct for construction asm related strings
pub struct ASMBuilder;

impl ASMBuilder {

    pub fn push(argument: &str) -> String {
        format!("{}", argument)
    }
    pub fn comment_line(argument: &str) -> String {
        format!("; {}\n", argument)
    }

    pub fn line(argument: &str) -> String {
        format!("{}{}\n", " ".repeat(0), argument)
    }

    pub fn line_ident(argument: &str) -> String {
        format!("{}{}\n", " ".repeat(4), argument)
    }

    pub fn ident(argument: &str) -> String {
        format!("{}{}", " ".repeat(4), argument)
    }
}