use std::fmt::{Display, Formatter};

/// A utility struct for construction asm related strings
pub struct ASMBuilder;

pub enum MovInstruction {
    Mov,
    MovQ,
    MovD,
}

impl Display for MovInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MovInstruction::Mov => write!(f, "mov"),
            MovInstruction::MovQ => write!(f, "movq"),
            MovInstruction::MovD => write!(f, "movd"),
        }
    }
}

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

    pub fn mov_x_ident_line<T: Display, P: Display>(destination: T, source: P, byte_size: Option<usize>) -> String {
        let source = source.to_string();
        let destination = destination.to_string();

        if source == destination {
            return String::new();
        }

        match byte_size {
            Some(8) => Self::mov_instruction_ident_line(MovInstruction::MovQ, destination, source),
            Some(4) => Self::mov_instruction_ident_line(MovInstruction::MovD, destination, source),
            _ => Self::mov_instruction_ident_line(MovInstruction::Mov, destination, source),
        }
    }

    fn mov_instruction_ident_line<T: Display, P: Display>(mov_instruction: MovInstruction, destination: T, source: P) -> String {
        let s = Self::mov_instruction_ident(mov_instruction, destination, source);

        return if s.ends_with("\n") {
            s
        } else {
            format!("{s}\n")
        };
    }

    pub fn mov_instruction_ident<T: Display, P: Display>(mov_instruction: MovInstruction, destination: T, source: P) -> String {
        let source = source.to_string();
        let destination = destination.to_string();

        if source == destination {
            return String::new();
        }

        format!("{}{} {}", " ".repeat(4), mov_instruction, ASMBuilder::comma_seperated(&destination, &source))
    }

    pub fn mov_ident_line<T: Display, P: Display>(destination: T, source: P) -> String {
        Self::mov_x_ident_line::<T, P>(destination, source, None)
    }

    pub fn comma_seperated(destination: &str, source: &str) -> String {
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