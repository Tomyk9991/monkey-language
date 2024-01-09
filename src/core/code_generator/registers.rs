use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::core::code_generator::ASMGenerateError;

#[derive(Debug, Clone)]
pub enum Bit64 {
    Rax,
    Rcx,
    Rdi,
    Rdx,
}

#[derive(Debug, Clone)]
pub enum Bit32 {
    Eax,
    Ecx,
    Edi,
    Edx,
}

#[derive(Debug, Clone)]
pub enum Bit16 {
    Ax,
    Cx,
    Di,
    Dx,
}

#[derive(Debug, Clone)]
pub enum Bit8 {
    Single(NibbleRegister),
    _Tuple(NibbleRegister, NibbleRegister),
}

#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum NibbleRegister {
    AH,
    AL,
    BH,
    BL,
    CH,
    CL,
    SPL,
    BPL,
    BIL,
    SIL,
    DH,
    DL,
}

#[derive(Debug, Clone)]
pub enum GeneralPurposeRegister {
    Bit64(Bit64),
    Bit32(Bit32),
    Bit16(Bit16),
    Bit8(Bit8),
}

impl FromStr for GeneralPurposeRegister {
    type Err = ASMGenerateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rax" => Ok(GeneralPurposeRegister::Bit64(Bit64::Rax)),
            "rcx" => Ok(GeneralPurposeRegister::Bit64(Bit64::Rcx)),
            "rdi" => Ok(GeneralPurposeRegister::Bit64(Bit64::Rdi)),
            "rdx" => Ok(GeneralPurposeRegister::Bit64(Bit64::Rdx)),
            "eax" => Ok(GeneralPurposeRegister::Bit32(Bit32::Eax)),
            "ecx" => Ok(GeneralPurposeRegister::Bit32(Bit32::Ecx)),
            "edi" => Ok(GeneralPurposeRegister::Bit32(Bit32::Edi)),
            "edx" => Ok(GeneralPurposeRegister::Bit32(Bit32::Edx)),
            "ax" => Ok(GeneralPurposeRegister::Bit16(Bit16::Ax)),
            "cx" => Ok(GeneralPurposeRegister::Bit16(Bit16::Cx)),
            "di" => Ok(GeneralPurposeRegister::Bit16(Bit16::Di)),
            "dx" => Ok(GeneralPurposeRegister::Bit16(Bit16::Dx)),
            "ah" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::AH))),
            "al" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::AL))),
            "bh" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::BH))),
            "bl" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::BL))),
            "ch" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::CH))),
            "cl" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::CL))),
            "spl" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::SPL))),
            "bpl" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::BPL))),
            "bil" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::BIL))),
            "sil" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::SIL))),
            "dh" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::DH))),
            "dl" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::DL))),
            a => { Err(ASMGenerateError::InternalError(format!("Could not convert `{a}` into a register"))) }
        }
    }
}


impl GeneralPurposeRegister {
    pub fn to_64_bit_register(&self) -> GeneralPurposeRegister {
        match self {
            GeneralPurposeRegister::Bit64(v) => GeneralPurposeRegister::Bit64(v.clone()),
            GeneralPurposeRegister::Bit32(v) => {
                match v {
                    Bit32::Eax => GeneralPurposeRegister::Bit64(Bit64::Rax),
                    Bit32::Ecx => GeneralPurposeRegister::Bit64(Bit64::Rcx),
                    Bit32::Edi => GeneralPurposeRegister::Bit64(Bit64::Rdi),
                    Bit32::Edx => GeneralPurposeRegister::Bit64(Bit64::Rdx),
                }
            }
            GeneralPurposeRegister::Bit16(v) => {
                match v {
                    Bit16::Ax => GeneralPurposeRegister::Bit64(Bit64::Rax),
                    Bit16::Cx => GeneralPurposeRegister::Bit64(Bit64::Rcx),
                    Bit16::Di => GeneralPurposeRegister::Bit64(Bit64::Rdi),
                    Bit16::Dx => GeneralPurposeRegister::Bit64(Bit64::Rdx),
                }
            }
            GeneralPurposeRegister::Bit8(v) => {
                match v {
                    Bit8::Single(v) =>  GeneralPurposeRegister::Bit64(v.to_64_bit_register()),
                    Bit8::_Tuple(v1, _) => GeneralPurposeRegister::Bit64(v1.to_64_bit_register())
                }
            }
        }
    }
}

impl NibbleRegister {
    pub fn to_64_bit_register(&self) -> Bit64 {
        match self {
            NibbleRegister::AH => Bit64::Rax,
            NibbleRegister::AL => Bit64::Rax,
            NibbleRegister::BH => Bit64::Rcx,
            NibbleRegister::BL => Bit64::Rcx,
            NibbleRegister::CH => Bit64::Rdi,
            NibbleRegister::CL => Bit64::Rdi,
            NibbleRegister::SPL => Bit64::Rdx,
            NibbleRegister::BPL => Bit64::Rdx,
            NibbleRegister::BIL => Bit64::Rdx,
            NibbleRegister::SIL => Bit64::Rdx,
            NibbleRegister::DH => Bit64::Rdx,
            NibbleRegister::DL => Bit64::Rdx,
        }
    }
}

impl From<Bit64> for GeneralPurposeRegister {
    fn from(value: Bit64) -> Self {
        GeneralPurposeRegister::Bit64(value)
    }
}

impl From<Bit32> for GeneralPurposeRegister {
    fn from(value: Bit32) -> Self {
        GeneralPurposeRegister::Bit32(value)
    }
}

impl Display for NibbleRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            NibbleRegister::AH => "ah",
            NibbleRegister::AL => "al",
            NibbleRegister::BH => "bh",
            NibbleRegister::BL => "bl",
            NibbleRegister::CH => "ch",
            NibbleRegister::CL => "cl",
            NibbleRegister::SPL => "spl",
            NibbleRegister::BPL => "bpl",
            NibbleRegister::BIL => "bil",
            NibbleRegister::SIL => "sil",
            NibbleRegister::DH => "dh",
            NibbleRegister::DL => "dl",
        })
    }
}

impl Display for Bit8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Bit8::Single(b) => write!(f, "{}", b),
            Bit8::_Tuple(b1, b2) => write!(f, "{}, {}", b1, b2)
        }
    }
}

impl Display for Bit16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Bit16::Ax => "ax",
            Bit16::Cx => "cx",
            Bit16::Di => "di",
            Bit16::Dx => "dx",
        })
    }
}


impl Display for Bit32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Bit32::Eax => "eax",
            Bit32::Ecx => "ecx",
            Bit32::Edi => "edi",
            Bit32::Edx => "edx",
        })
    }
}

impl Display for Bit64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Bit64::Rax => "rax",
            Bit64::Rcx => "rcx",
            Bit64::Rdi => "rdi",
            Bit64::Rdx => "rdx",
        })
    }
}

impl Display for GeneralPurposeRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            GeneralPurposeRegister::Bit64(b) => b.to_string(),
            GeneralPurposeRegister::Bit32(b) => b.to_string(),
            GeneralPurposeRegister::Bit16(b) => b.to_string(),
            GeneralPurposeRegister::Bit8(b) => b.to_string()
        })
    }
}