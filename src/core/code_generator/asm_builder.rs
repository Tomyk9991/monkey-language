use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::core::code_generator::registers::{Bit64, GeneralPurposeRegister, GeneralPurposeRegisterIterator};

/// A utility struct for construction asm related strings
pub struct ASMBuilder;

#[derive(PartialEq)]
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
    pub fn comment_line(argument: &str) -> String {
        format!("; {}\n", argument)
    }

    pub fn line(argument: &str) -> String {
        format!("{}{}\n", " ".repeat(0), argument)
    }

    pub fn ident_line(argument: &str) -> String {
        format!("{}{}\n", " ".repeat(4), argument)
    }

    pub fn push_registers(ignore_registers: &[&GeneralPurposeRegister]) -> String {
        let mut general_purpose_registers = GeneralPurposeRegisterIterator::new(GeneralPurposeRegister::Bit64(Bit64::Rax))
            .collect::<Vec<_>>();

        general_purpose_registers.insert(0, GeneralPurposeRegister::Bit64(Bit64::Rax));

        let mut target = String::new();
        target += &Self::ident(&Self::comment_line("PushQ"));

        for register in general_purpose_registers {
            if ignore_registers.iter().any(|a| &&register.to_size_register(&a.size()) == a) {
                continue;
            }
            target += &Self::ident_line(&format!("push {}", register));
        }

        target
    }

    pub fn pop_registers(ignore_registers: &[&GeneralPurposeRegister]) -> String {
        let mut general_purpose_registers = GeneralPurposeRegisterIterator::new(GeneralPurposeRegister::Bit64(Bit64::Rax))
            .collect::<Vec<GeneralPurposeRegister>>();
        general_purpose_registers.insert(0, GeneralPurposeRegister::Bit64(Bit64::Rax));
        general_purpose_registers.reverse();

        let mut target = String::new();
        target += &Self::ident(&Self::comment_line("PopQ"));

        for register in &general_purpose_registers {
            if ignore_registers.iter().any(|a| &&register.to_size_register(&a.size()) == a) {
                continue;
            }

            target += &Self::ident_line(&format!("pop {register}"));
        }

        target
    }

    pub fn mov_line<T: Display, P: Display>(destination: T, source: P) -> String {
        let source = source.to_string();
        let destination = destination.to_string();

        if source == destination {
            return String::new();
        }

        format!("mov {}\n", ASMBuilder::comma_seperated(&destination, &source))
    }

    pub fn mov_x_ident_line<T: Display, P: Display>(destination: T, source: P, byte_size: Option<usize>) -> String {
        let source = source.to_string();
        let destination = destination.to_string();

        if source == destination {
            return String::new();
        }

        if !source.starts_with("xmm") && !destination.starts_with("xmm") {
            return Self::mov_instruction_ident_line(MovInstruction::Mov, destination, source);
        }

        match byte_size {
            _ if source.starts_with("xmm") && destination.starts_with("xmm") => Self::mov_instruction_ident_line(MovInstruction::MovQ, destination, source),
            Some(8) => Self::mov_instruction_ident_line(MovInstruction::MovQ, destination, source),
            Some(4) => Self::mov_instruction_ident_line(MovInstruction::MovD, destination, source),
            _ => Self::mov_instruction_ident_line(MovInstruction::Mov, destination, source),
        }
    }

    fn mov_instruction_ident_line<T: Display, P: Display>(mov_instruction: MovInstruction, destination: T, source: P) -> String {
        let s = Self::mov_instruction_ident(mov_instruction, destination, source);

        if s.ends_with('\n') || s.is_empty() {
            s
        } else {
            format!("{s}\n")
        }
    }

    pub fn mov_instruction_ident<T: Display, P: Display>(mov_instruction: MovInstruction, destination: T, source: P) -> String {
        let source = source.to_string();
        let destination = destination.to_string();

        if let (Ok(dest), Ok(sour)) = (GeneralPurposeRegister::from_str(&destination), GeneralPurposeRegister::from_str(&source)) {
            if mov_instruction == MovInstruction::Mov && dest.to_64_bit_register() == sour.to_64_bit_register() {
                return String::new();
            }
        }

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