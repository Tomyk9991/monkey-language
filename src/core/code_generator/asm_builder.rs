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

    #[allow(unused)]
    pub fn ident_comment_line(comment: &str) -> String {
        format!("{} ; {}\n", " ".repeat(4), comment)
    }

    pub fn ident(argument: &str) -> String {
        format!("{}{}", " ".repeat(4), argument)
    }
}