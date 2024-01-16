use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::core::code_generator::ASMGenerateError;

#[derive(Debug, Clone)]
pub enum Bit64 {
    Rax,
    Rcx,
    Rdi,
    Rdx,
    R8,
    R9
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
    Float(FloatRegister),
}

#[derive(Debug, Clone)]
pub enum FloatRegister {
    Xmm0,
    Xmm1,
    Xmm2,
    Xmm3,
    Xmm4,
    Xmm5,
    Xmm6,
    Xmm7,
}

impl FromStr for FloatRegister {
    type Err = ASMGenerateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "xmm0" => Ok(FloatRegister::Xmm0),
            "xmm1" => Ok(FloatRegister::Xmm1),
            "xmm2" => Ok(FloatRegister::Xmm2),
            "xmm3" => Ok(FloatRegister::Xmm3),
            "xmm4" => Ok(FloatRegister::Xmm4),
            "xmm5" => Ok(FloatRegister::Xmm5),
            "xmm6" => Ok(FloatRegister::Xmm6),
            "xmm7" => Ok(FloatRegister::Xmm7),
            a => { Err(ASMGenerateError::InternalError(format!("Could not convert `{a}` into a register"))) }
        }
    }

}

impl FromStr for GeneralPurposeRegister {
    type Err = ASMGenerateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rax" => Ok(GeneralPurposeRegister::Bit64(Bit64::Rax)),
            "rcx" => Ok(GeneralPurposeRegister::Bit64(Bit64::Rcx)),
            "rdi" => Ok(GeneralPurposeRegister::Bit64(Bit64::Rdi)),
            "rdx" => Ok(GeneralPurposeRegister::Bit64(Bit64::Rdx)),
            "r9" => Ok(GeneralPurposeRegister::Bit64(Bit64::R9)),
            "r8" => Ok(GeneralPurposeRegister::Bit64(Bit64::R8)),
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


pub struct GeneralPurposeRegisterIterator {
    current: GeneralPurposeRegister,
}

impl GeneralPurposeRegisterIterator {
    pub fn new(register: GeneralPurposeRegister) -> GeneralPurposeRegisterIterator {
        GeneralPurposeRegisterIterator {
            current: register,
        }
    }

    pub fn current(&self) -> GeneralPurposeRegister {
        self.current.clone()
    }
}


impl Iterator for GeneralPurposeRegisterIterator {
    type Item = GeneralPurposeRegister;

    fn next(&mut self) -> Option<Self::Item> {
        return match &self.current {
            GeneralPurposeRegister::Bit64(a) => {
                match a {
                    Bit64::Rax => { self.current = GeneralPurposeRegister::Bit64(Bit64::Rcx);  Some(GeneralPurposeRegister::Bit64(Bit64::Rcx)) },
                    Bit64::Rcx => { self.current = GeneralPurposeRegister::Bit64(Bit64::Rdi);  Some(GeneralPurposeRegister::Bit64(Bit64::Rdi)) },
                    Bit64::Rdi => { self.current = GeneralPurposeRegister::Bit64(Bit64::Rdx);  Some(GeneralPurposeRegister::Bit64(Bit64::Rdx)) },
                    Bit64::Rdx => None,
                    Bit64::R8 => None,
                    Bit64::R9 => None
                }
            }
            GeneralPurposeRegister::Bit32(a) => {
                match a {
                    Bit32::Eax => { self.current = GeneralPurposeRegister::Bit32(Bit32::Ecx); Some(GeneralPurposeRegister::Bit32(Bit32::Ecx)) },
                    Bit32::Ecx => { self.current = GeneralPurposeRegister::Bit32(Bit32::Edi); Some(GeneralPurposeRegister::Bit32(Bit32::Edi)) },
                    Bit32::Edi => { self.current = GeneralPurposeRegister::Bit32(Bit32::Edx); Some(GeneralPurposeRegister::Bit32(Bit32::Edx)) },
                    Bit32::Edx => None,
                }
            }
            GeneralPurposeRegister::Bit16(a) => {
                match a {
                    Bit16::Ax => { self.current = GeneralPurposeRegister::Bit16(Bit16::Cx); Some(GeneralPurposeRegister::Bit16(Bit16::Cx)) },
                    Bit16::Cx => { self.current = GeneralPurposeRegister::Bit16(Bit16::Di); Some(GeneralPurposeRegister::Bit16(Bit16::Di)) },
                    Bit16::Di => { self.current = GeneralPurposeRegister::Bit16(Bit16::Dx); Some(GeneralPurposeRegister::Bit16(Bit16::Dx)) },
                    Bit16::Dx => None,
                }
            }
            GeneralPurposeRegister::Bit8(a) => {
                match a {
                    Bit8::Single(n) => {
                        match n {
                            NibbleRegister::AH =>  { self.current = GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::AL)); Some(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::AL))) },
                            NibbleRegister::AL =>  { self.current = GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::BH)); Some(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::BH))) },
                            NibbleRegister::BH =>  { self.current = GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::BL)); Some(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::BL))) },
                            NibbleRegister::BL =>  { self.current = GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::CH)); Some(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::CH))) },
                            NibbleRegister::CH =>  { self.current = GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::CL)); Some(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::CL))) },
                            NibbleRegister::CL =>  { self.current = GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::SPL)); Some(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::SPL))) },
                            NibbleRegister::SPL => { self.current = GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::BPL)); Some(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::BPL))) },
                            NibbleRegister::BPL => { self.current = GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::BIL)); Some(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::BIL))) },
                            NibbleRegister::BIL => { self.current = GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::SIL)); Some(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::SIL))) },
                            NibbleRegister::SIL => { self.current = GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::DH)); Some(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::DH))) },
                            NibbleRegister::DH =>  { self.current = GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::DL)); Some(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::DL))) },
                            NibbleRegister::DL => None,
                        }
                    }
                    _ => None
                }
            }
            GeneralPurposeRegister::Float(f) => {
                match f {
                    FloatRegister::Xmm0 => { self.current = GeneralPurposeRegister::Float(FloatRegister::Xmm1); Some(GeneralPurposeRegister::Float(FloatRegister::Xmm1)) },
                    FloatRegister::Xmm1 => { self.current = GeneralPurposeRegister::Float(FloatRegister::Xmm2); Some(GeneralPurposeRegister::Float(FloatRegister::Xmm2)) },
                    FloatRegister::Xmm2 => { self.current = GeneralPurposeRegister::Float(FloatRegister::Xmm3); Some(GeneralPurposeRegister::Float(FloatRegister::Xmm3)) },
                    FloatRegister::Xmm3 => { self.current = GeneralPurposeRegister::Float(FloatRegister::Xmm4); Some(GeneralPurposeRegister::Float(FloatRegister::Xmm4)) },
                    FloatRegister::Xmm4 => { self.current = GeneralPurposeRegister::Float(FloatRegister::Xmm5); Some(GeneralPurposeRegister::Float(FloatRegister::Xmm5)) },
                    FloatRegister::Xmm5 => { self.current = GeneralPurposeRegister::Float(FloatRegister::Xmm6); Some(GeneralPurposeRegister::Float(FloatRegister::Xmm6)) },
                    FloatRegister::Xmm6 => { self.current = GeneralPurposeRegister::Float(FloatRegister::Xmm7); Some(GeneralPurposeRegister::Float(FloatRegister::Xmm7)) },
                    FloatRegister::Xmm7 => None,
                }
            }
        };
    }
}



impl GeneralPurposeRegister {
    pub fn iter_from_byte_size(size: usize) -> Result<GeneralPurposeRegisterIterator, ASMGenerateError> {
        match size {
            1 => Ok(GeneralPurposeRegisterIterator::new(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::AH)))),
            2 => Ok(GeneralPurposeRegisterIterator::new(GeneralPurposeRegister::Bit16(Bit16::Ax))),
            4 => Ok(GeneralPurposeRegisterIterator::new(GeneralPurposeRegister::Bit32(Bit32::Eax))),
            8 => Ok(GeneralPurposeRegisterIterator::new(GeneralPurposeRegister::Bit64(Bit64::Rax))),
            _ => Err(ASMGenerateError::InternalError(format!("Could not convert `{}` into a register", size))),
        }
    }

    pub fn iter_float_register() -> Result<GeneralPurposeRegisterIterator, ASMGenerateError> {
        return Ok(GeneralPurposeRegisterIterator::new(GeneralPurposeRegister::Float(FloatRegister::Xmm0)));
    }

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
            GeneralPurposeRegister::Float(f) => { unreachable!("{} to 64 bit register expected", f) }
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

impl From<FloatRegister> for GeneralPurposeRegister {
    fn from(value: FloatRegister) -> Self { GeneralPurposeRegister::Float(value) }
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

impl Display for FloatRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            FloatRegister::Xmm0 => "xmm0",
            FloatRegister::Xmm1 => "xmm1",
            FloatRegister::Xmm2 => "xmm2",
            FloatRegister::Xmm3 => "xmm3",
            FloatRegister::Xmm4 => "xmm4",
            FloatRegister::Xmm5 => "xmm5",
            FloatRegister::Xmm6 => "xmm6",
            FloatRegister::Xmm7 => "xmm7",
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
            Bit64::R9 => "r9",
            Bit64::R8 => "r8",
        })
    }
}

impl Display for GeneralPurposeRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            GeneralPurposeRegister::Bit64(b) => b.to_string(),
            GeneralPurposeRegister::Bit32(b) => b.to_string(),
            GeneralPurposeRegister::Bit16(b) => b.to_string(),
            GeneralPurposeRegister::Bit8(b) => b.to_string(),
            GeneralPurposeRegister::Float(b) => b.to_string(),
        })
    }
}