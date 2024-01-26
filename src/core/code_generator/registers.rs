use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::core::code_generator::ASMGenerateError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Bit64 {
    Rax,
    Rcx,
    Rdi,
    Rdx,
    R15,
    R8,
    R9,
    /// Special register, used to cache something temporarily
    R14,
    /// Special register, used to cache something temporarily
    R13,
    /// Special register, used to cache something temporarily
    R12,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Bit32 {
    Eax,
    Ecx,
    Edi,
    Edx,
    R15d,
    R8d,
    R9d,
    R14d,
    R13d,
    R12d,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Bit16 {
    Ax,
    Cx,
    Di,
    Dx,
    R15w,
    R14w,
    R13w,
    R12w,
    R8w,
    R9w,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Bit8 {
    Single(NibbleRegister),
    _Tuple(NibbleRegister, NibbleRegister),
}

#[derive(Debug, Clone, Eq, PartialEq)]
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
    DIL,
    SIL,
    DH,
    DL,
    R15b,
    R14b,
    R13b,
    R12b,
    R8b,
    R9b,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GeneralPurposeRegister {
    Bit64(Bit64),
    Bit32(Bit32),
    Bit16(Bit16),
    Bit8(Bit8),
    Float(FloatRegister),
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FloatRegister {
    Xmm0,
    Xmm1,
    Xmm2,
    Xmm3,
    Xmm4,
    Xmm5,
    Xmm6,
    /// Special use in this compiler. Used for only for float to float conversion
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
            a => { Err(ASMGenerateError::InternalError(format!("Float: Could not convert `{a}` into a register"))) }
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
            "r15" => Ok(GeneralPurposeRegister::Bit64(Bit64::R15)),
            "r9" => Ok(GeneralPurposeRegister::Bit64(Bit64::R9)),
            "r8" => Ok(GeneralPurposeRegister::Bit64(Bit64::R8)),
            "eax" => Ok(GeneralPurposeRegister::Bit32(Bit32::Eax)),
            "ecx" => Ok(GeneralPurposeRegister::Bit32(Bit32::Ecx)),
            "edi" => Ok(GeneralPurposeRegister::Bit32(Bit32::Edi)),
            "edx" => Ok(GeneralPurposeRegister::Bit32(Bit32::Edx)),
            "r15d" => Ok(GeneralPurposeRegister::Bit32(Bit32::R15d)),
            "r9d" => Ok(GeneralPurposeRegister::Bit32(Bit32::R9d)),
            "r8d" => Ok(GeneralPurposeRegister::Bit32(Bit32::R8d)),
            "ax" => Ok(GeneralPurposeRegister::Bit16(Bit16::Ax)),
            "cx" => Ok(GeneralPurposeRegister::Bit16(Bit16::Cx)),
            "di" => Ok(GeneralPurposeRegister::Bit16(Bit16::Di)),
            "dx" => Ok(GeneralPurposeRegister::Bit16(Bit16::Dx)),
            "r15w" => Ok(GeneralPurposeRegister::Bit16(Bit16::R15w)),
            "r14w" => Ok(GeneralPurposeRegister::Bit16(Bit16::R14w)),
            "r13w" => Ok(GeneralPurposeRegister::Bit16(Bit16::R13w)),
            "r12w" => Ok(GeneralPurposeRegister::Bit16(Bit16::R12w)),
            "r9w" => Ok(GeneralPurposeRegister::Bit16(Bit16::R9w)),
            "r8w" => Ok(GeneralPurposeRegister::Bit16(Bit16::R8w)),
            "r15b" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R15b))),
            "r14b" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R14b))),
            "r13b" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R13b))),
            "r12b" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R12b))),
            "r9b" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R9b))),
            "r8b" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R8b))),
            "ah" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::AH))),
            "al" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::AL))),
            "bh" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::BH))),
            "bl" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::BL))),
            "ch" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::CH))),
            "cl" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::CL))),
            "spl" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::SPL))),
            "bpl" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::BPL))),
            "dil" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::DIL))),
            "sil" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::SIL))),
            "dh" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::DH))),
            "dl" => Ok(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::DL))),
            "xmm0" => Ok(GeneralPurposeRegister::Float(FloatRegister::Xmm0)),
            "xmm1" => Ok(GeneralPurposeRegister::Float(FloatRegister::Xmm1)),
            "xmm2" => Ok(GeneralPurposeRegister::Float(FloatRegister::Xmm2)),
            "xmm3" => Ok(GeneralPurposeRegister::Float(FloatRegister::Xmm3)),
            "xmm4" => Ok(GeneralPurposeRegister::Float(FloatRegister::Xmm4)),
            "xmm5" => Ok(GeneralPurposeRegister::Float(FloatRegister::Xmm5)),
            "xmm6" => Ok(GeneralPurposeRegister::Float(FloatRegister::Xmm6)),
            "xmm7" => Ok(GeneralPurposeRegister::Float(FloatRegister::Xmm7)),
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
        match &self.current {
            GeneralPurposeRegister::Bit64(a) => {
                match a {
                    Bit64::Rax => {
                        self.current = GeneralPurposeRegister::Bit64(Bit64::Rcx);
                        Some(GeneralPurposeRegister::Bit64(Bit64::Rcx))
                    }
                    Bit64::Rcx => {
                        self.current = GeneralPurposeRegister::Bit64(Bit64::Rdi);
                        Some(GeneralPurposeRegister::Bit64(Bit64::Rdi))
                    }
                    Bit64::Rdi => {
                        self.current = GeneralPurposeRegister::Bit64(Bit64::Rdx);
                        Some(GeneralPurposeRegister::Bit64(Bit64::Rdx))
                    },
                    _ => None
                }
            }
            GeneralPurposeRegister::Bit32(a) => {
                match a {
                    Bit32::Eax => {
                        self.current = GeneralPurposeRegister::Bit32(Bit32::Ecx);
                        Some(GeneralPurposeRegister::Bit32(Bit32::Ecx))
                    }
                    Bit32::Ecx => {
                        self.current = GeneralPurposeRegister::Bit32(Bit32::Edi);
                        Some(GeneralPurposeRegister::Bit32(Bit32::Edi))
                    }
                    Bit32::Edi => {
                        self.current = GeneralPurposeRegister::Bit32(Bit32::Edx);
                        Some(GeneralPurposeRegister::Bit32(Bit32::Edx))
                    },
                    _ => None
                }
            }
            GeneralPurposeRegister::Bit16(a) => {
                match a {
                    Bit16::Ax => {
                        self.current = GeneralPurposeRegister::Bit16(Bit16::Cx);
                        Some(GeneralPurposeRegister::Bit16(Bit16::Cx))
                    }
                    Bit16::Cx => {
                        self.current = GeneralPurposeRegister::Bit16(Bit16::Di);
                        Some(GeneralPurposeRegister::Bit16(Bit16::Di))
                    }
                    Bit16::Di => {
                        self.current = GeneralPurposeRegister::Bit16(Bit16::Dx);
                        Some(GeneralPurposeRegister::Bit16(Bit16::Dx))
                    },
                    _ => None
                }
            }
            GeneralPurposeRegister::Bit8(a) => {
                match a {
                    Bit8::Single(n) => {
                        match n {
                            NibbleRegister::AL => {
                                self.current = GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::BL));
                                Some(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::BL)))
                            }
                            NibbleRegister::BL => {
                                self.current = GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::CL));
                                Some(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::CL)))
                            },
                            NibbleRegister::CL => {
                                self.current = GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::DIL));
                                Some(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::DIL)))
                            },
                            NibbleRegister::DIL => {
                                self.current = GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::BH));
                                Some(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::BH)))
                            },
                            _ => None,
                        }
                    }
                    _ => None
                }
            }
            GeneralPurposeRegister::Float(f) => {
                match f {
                    FloatRegister::Xmm0 => {
                        self.current = GeneralPurposeRegister::Float(FloatRegister::Xmm1);
                        Some(GeneralPurposeRegister::Float(FloatRegister::Xmm1))
                    }
                    FloatRegister::Xmm1 => {
                        self.current = GeneralPurposeRegister::Float(FloatRegister::Xmm2);
                        Some(GeneralPurposeRegister::Float(FloatRegister::Xmm2))
                    }
                    FloatRegister::Xmm2 => {
                        self.current = GeneralPurposeRegister::Float(FloatRegister::Xmm3);
                        Some(GeneralPurposeRegister::Float(FloatRegister::Xmm3))
                    }
                    FloatRegister::Xmm3 => {
                        self.current = GeneralPurposeRegister::Float(FloatRegister::Xmm4);
                        Some(GeneralPurposeRegister::Float(FloatRegister::Xmm4))
                    }
                    FloatRegister::Xmm4 => {
                        self.current = GeneralPurposeRegister::Float(FloatRegister::Xmm5);
                        Some(GeneralPurposeRegister::Float(FloatRegister::Xmm5))
                    }
                    FloatRegister::Xmm5 => {
                        self.current = GeneralPurposeRegister::Float(FloatRegister::Xmm6);
                        Some(GeneralPurposeRegister::Float(FloatRegister::Xmm6))
                    }
                    FloatRegister::Xmm6 => {
                        self.current = GeneralPurposeRegister::Float(FloatRegister::Xmm7);
                        Some(GeneralPurposeRegister::Float(FloatRegister::Xmm7))
                    }
                    FloatRegister::Xmm7 => None,
                }
            }
        }
    }
}


impl GeneralPurposeRegister {
    pub fn iter_from_byte_size(size: usize) -> Result<GeneralPurposeRegisterIterator, ASMGenerateError> {
        match size {
            1 => Ok(GeneralPurposeRegisterIterator::new(GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::AL)))),
            2 => Ok(GeneralPurposeRegisterIterator::new(GeneralPurposeRegister::Bit16(Bit16::Ax))),
            4 => Ok(GeneralPurposeRegisterIterator::new(GeneralPurposeRegister::Bit32(Bit32::Eax))),
            8 => Ok(GeneralPurposeRegisterIterator::new(GeneralPurposeRegister::Bit64(Bit64::Rax))),
            _ => Err(ASMGenerateError::InternalError(format!("Could not convert `{}` into a register", size))),
        }
    }

    pub fn size(&self) -> ByteSize {
       match self {
           GeneralPurposeRegister::Bit64(_) => ByteSize::_8,
           GeneralPurposeRegister::Float(_) => ByteSize::_8,
           GeneralPurposeRegister::Bit32(_) => ByteSize::_4,
           GeneralPurposeRegister::Bit16(_) => ByteSize::_2,
           GeneralPurposeRegister::Bit8(_) => ByteSize::_1,
       }
    }

    pub fn iter_float_register() -> Result<GeneralPurposeRegisterIterator, ASMGenerateError> {
        Ok(GeneralPurposeRegisterIterator::new(GeneralPurposeRegister::Float(FloatRegister::Xmm0)))
    }

    pub fn to_16_bit_register(&self) -> GeneralPurposeRegister {
        match self {
            GeneralPurposeRegister::Bit64(v) => {
                match v {
                    Bit64::Rax => GeneralPurposeRegister::Bit16(Bit16::Ax),
                    Bit64::Rcx => GeneralPurposeRegister::Bit16(Bit16::Cx),
                    Bit64::Rdi => GeneralPurposeRegister::Bit16(Bit16::Di),
                    Bit64::Rdx => GeneralPurposeRegister::Bit16(Bit16::Dx),
                    Bit64::R15 => GeneralPurposeRegister::Bit16(Bit16::R15w),
                    Bit64::R14 => GeneralPurposeRegister::Bit16(Bit16::R14w),
                    Bit64::R13 => GeneralPurposeRegister::Bit16(Bit16::R13w),
                    Bit64::R12 => GeneralPurposeRegister::Bit16(Bit16::R12w),
                    Bit64::R8 => GeneralPurposeRegister::Bit16(Bit16::R8w),
                    Bit64::R9 => GeneralPurposeRegister::Bit16(Bit16::R9w),
                }
            },
            GeneralPurposeRegister::Bit32(v) => {
                match v {
                    Bit32::Eax => GeneralPurposeRegister::Bit16(Bit16::Ax),
                    Bit32::Ecx => GeneralPurposeRegister::Bit16(Bit16::Cx),
                    Bit32::Edi => GeneralPurposeRegister::Bit16(Bit16::Di),
                    Bit32::Edx => GeneralPurposeRegister::Bit16(Bit16::Dx),
                    Bit32::R15d => GeneralPurposeRegister::Bit16(Bit16::R15w),
                    Bit32::R14d => GeneralPurposeRegister::Bit16(Bit16::R14w),
                    Bit32::R13d => GeneralPurposeRegister::Bit16(Bit16::R13w),
                    Bit32::R12d => GeneralPurposeRegister::Bit16(Bit16::R12w),
                    Bit32::R8d => GeneralPurposeRegister::Bit16(Bit16::R8w),
                    Bit32::R9d => GeneralPurposeRegister::Bit16(Bit16::R9w),
                }
            },
            GeneralPurposeRegister::Bit16(v) => GeneralPurposeRegister::Bit16(v.clone()),
            GeneralPurposeRegister::Bit8(v) => {
                match v {
                    Bit8::Single(NibbleRegister::AH) => GeneralPurposeRegister::Bit16(Bit16::Ax),
                    Bit8::Single(NibbleRegister::AL) => GeneralPurposeRegister::Bit16(Bit16::Ax),
                    Bit8::Single(NibbleRegister::BH) => GeneralPurposeRegister::Bit16(Bit16::Cx),
                    Bit8::Single(NibbleRegister::BL) => GeneralPurposeRegister::Bit16(Bit16::Cx),
                    Bit8::Single(NibbleRegister::CH) => GeneralPurposeRegister::Bit16(Bit16::Cx),
                    Bit8::Single(NibbleRegister::CL) => GeneralPurposeRegister::Bit16(Bit16::Cx),
                    Bit8::Single(NibbleRegister::DIL) => GeneralPurposeRegister::Bit16(Bit16::Di),
                    Bit8::Single(NibbleRegister::DH) => GeneralPurposeRegister::Bit16(Bit16::Dx),
                    Bit8::Single(NibbleRegister::DL) => GeneralPurposeRegister::Bit16(Bit16::Dx),
                    _ => unimplemented!("Not needed yet")
                }
            },
            GeneralPurposeRegister::Float(_) => unreachable!("Float register cannot be converted to 16s bit register"),
        }
    }

    pub fn to_8_bit_register(&self) -> GeneralPurposeRegister {
        match self {
            GeneralPurposeRegister::Bit64(v) => {
                match v {
                    Bit64::Rax => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::AL)),
                    Bit64::Rcx => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::CL)),
                    Bit64::Rdi => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::DIL)),
                    Bit64::Rdx => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::DL)),
                    Bit64::R15 => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R15b)),
                    Bit64::R14 => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R14b)),
                    Bit64::R13 => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R13b)),
                    Bit64::R12 => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R12b)),
                    Bit64::R8 => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R8b)),
                    Bit64::R9 => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R9b)),
                }
            }
            GeneralPurposeRegister::Bit32(v) => {
                match v {
                    Bit32::Eax => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::AL)),
                    Bit32::Ecx => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::CL)),
                    Bit32::Edi => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::DIL)),
                    Bit32::Edx => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::DL)),
                    Bit32::R15d => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R15b)),
                    Bit32::R14d => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R14b)),
                    Bit32::R13d => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R13b)),
                    Bit32::R12d => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R12b)),
                    Bit32::R8d => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R8b)),
                    Bit32::R9d => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R9b)),
                }
            }
            GeneralPurposeRegister::Bit16(v) => {
                match v {
                    Bit16::Ax => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::AL)),
                    Bit16::Cx => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::CL)),
                    Bit16::Di => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::DIL)),
                    Bit16::Dx => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::DL)),
                    Bit16::R15w => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R15b)),
                    Bit16::R14w => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R14b)),
                    Bit16::R13w => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R13b)),
                    Bit16::R12w => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R12b)),
                    Bit16::R8w => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R8b)),
                    Bit16::R9w => GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::R9b)),

                }
            }
            GeneralPurposeRegister::Bit8(v) => GeneralPurposeRegister::Bit8(v.clone()),
            GeneralPurposeRegister::Float(_) => unreachable!("Float register cannot be converted to 8 bit register"),
        }
    }

    pub fn to_32_bit_register(&self) -> GeneralPurposeRegister {
        match self {
            GeneralPurposeRegister::Bit64(v) => {
                match v {
                    Bit64::Rax => GeneralPurposeRegister::Bit32(Bit32::Eax),
                    Bit64::Rcx => GeneralPurposeRegister::Bit32(Bit32::Ecx),
                    Bit64::Rdi => GeneralPurposeRegister::Bit32(Bit32::Edi),
                    Bit64::Rdx => GeneralPurposeRegister::Bit32(Bit32::Edx),
                    Bit64::R15 => GeneralPurposeRegister::Bit32(Bit32::R15d),
                    Bit64::R8 => GeneralPurposeRegister::Bit32(Bit32::R8d),
                    Bit64::R9 => GeneralPurposeRegister::Bit32(Bit32::R9d),
                    Bit64::R14 => GeneralPurposeRegister::Bit32(Bit32::R14d),
                    Bit64::R13 => GeneralPurposeRegister::Bit32(Bit32::R13d),
                    Bit64::R12 => GeneralPurposeRegister::Bit32(Bit32::R12d)
                }
            }
            GeneralPurposeRegister::Bit32(v) => GeneralPurposeRegister::Bit32(v.clone()),
            GeneralPurposeRegister::Bit16(v) => {
                match v {
                    Bit16::Ax => GeneralPurposeRegister::Bit32(Bit32::Eax),
                    Bit16::Cx => GeneralPurposeRegister::Bit32(Bit32::Ecx),
                    Bit16::Di => GeneralPurposeRegister::Bit32(Bit32::Edi),
                    Bit16::Dx => GeneralPurposeRegister::Bit32(Bit32::Edx),
                    Bit16::R15w => GeneralPurposeRegister::Bit32(Bit32::R15d),
                    Bit16::R14w => GeneralPurposeRegister::Bit32(Bit32::R14d),
                    Bit16::R13w => GeneralPurposeRegister::Bit32(Bit32::R13d),
                    Bit16::R12w => GeneralPurposeRegister::Bit32(Bit32::R12d),
                    Bit16::R9w => GeneralPurposeRegister::Bit32(Bit32::R9d),
                    Bit16::R8w => GeneralPurposeRegister::Bit32(Bit32::R8d),
                }
            }
            GeneralPurposeRegister::Bit8(v) => {
                match v {
                    Bit8::Single(v) => GeneralPurposeRegister::Bit32(v.to_32_bit_register()),
                    Bit8::_Tuple(v1, _) => GeneralPurposeRegister::Bit32(v1.to_32_bit_register())
                }
            }
            GeneralPurposeRegister::Float(f) => {
                match f {
                    FloatRegister::Xmm0 => GeneralPurposeRegister::Bit32(Bit32::Eax),
                    FloatRegister::Xmm1 => GeneralPurposeRegister::Bit32(Bit32::Ecx),
                    FloatRegister::Xmm2 => GeneralPurposeRegister::Bit32(Bit32::Edi),
                    FloatRegister::Xmm3 => GeneralPurposeRegister::Bit32(Bit32::Edx),
                    FloatRegister::Xmm4 => GeneralPurposeRegister::Bit32(Bit32::R8d),
                    FloatRegister::Xmm5 => GeneralPurposeRegister::Bit32(Bit32::R9d),
                    _ => todo!("Not enough general purpose registers")
                }
            }
        }
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
                    Bit32::R15d => GeneralPurposeRegister::Bit64(Bit64::R15),
                    Bit32::R8d => GeneralPurposeRegister::Bit64(Bit64::R8),
                    Bit32::R9d => GeneralPurposeRegister::Bit64(Bit64::R9),
                    Bit32::R14d => GeneralPurposeRegister::Bit64(Bit64::R14),
                    Bit32::R13d => GeneralPurposeRegister::Bit64(Bit64::R13),
                    Bit32::R12d => GeneralPurposeRegister::Bit64(Bit64::R12)
                }
            }
            GeneralPurposeRegister::Bit16(v) => {
                match v {
                    Bit16::Ax => GeneralPurposeRegister::Bit64(Bit64::Rax),
                    Bit16::Cx => GeneralPurposeRegister::Bit64(Bit64::Rcx),
                    Bit16::Di => GeneralPurposeRegister::Bit64(Bit64::Rdi),
                    Bit16::Dx => GeneralPurposeRegister::Bit64(Bit64::Rdx),
                    Bit16::R15w => GeneralPurposeRegister::Bit64(Bit64::R15),
                    Bit16::R14w => GeneralPurposeRegister::Bit64(Bit64::R14),
                    Bit16::R13w => GeneralPurposeRegister::Bit64(Bit64::R13),
                    Bit16::R12w => GeneralPurposeRegister::Bit64(Bit64::R12),
                    Bit16::R9w => GeneralPurposeRegister::Bit64(Bit64::R9),
                    Bit16::R8w => GeneralPurposeRegister::Bit64(Bit64::R8),
                }
            }
            GeneralPurposeRegister::Bit8(v) => {
                match v {
                    Bit8::Single(v) => GeneralPurposeRegister::Bit64(v.to_64_bit_register()),
                    Bit8::_Tuple(v1, _) => GeneralPurposeRegister::Bit64(v1.to_64_bit_register())
                }
            }
            GeneralPurposeRegister::Float(f) => {
                match f {
                    FloatRegister::Xmm0 => GeneralPurposeRegister::Bit64(Bit64::Rax),
                    FloatRegister::Xmm1 => GeneralPurposeRegister::Bit64(Bit64::Rcx),
                    FloatRegister::Xmm2 => GeneralPurposeRegister::Bit64(Bit64::Rdi),
                    FloatRegister::Xmm3 => GeneralPurposeRegister::Bit64(Bit64::Rdx),
                    FloatRegister::Xmm4 => GeneralPurposeRegister::Bit64(Bit64::R8),
                    FloatRegister::Xmm5 => GeneralPurposeRegister::Bit64(Bit64::R9),
                    _ => todo!("Not enough general purpose registers")
                }
            }
        }
    }

    /// Converts a general purpose register to the provided size
    pub fn to_size_register(&self, size: &ByteSize) -> GeneralPurposeRegister {
        match size {
            ByteSize::_8 => self.to_64_bit_register(),
            ByteSize::_4 => self.to_32_bit_register(),
            ByteSize::_2 => self.to_16_bit_register(),
            ByteSize::_1 => self.to_8_bit_register(),
        }
    }
}

pub enum ByteSize {
    _8,
    _4,
    _2,
    _1,
}

impl TryFrom<usize> for ByteSize {
    type Error = ASMGenerateError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            8 => Ok(ByteSize::_8),
            4 => Ok(ByteSize::_4),
            2 => Ok(ByteSize::_2),
            1 => Ok(ByteSize::_1),
            _ => Err(ASMGenerateError::InternalError("Something went wrong casting general purpose registers".to_string()))
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
            NibbleRegister::DIL => Bit64::Rdx,
            NibbleRegister::SIL => Bit64::Rdx,
            NibbleRegister::DH => Bit64::Rdx,
            NibbleRegister::DL => Bit64::Rdx,
            NibbleRegister::R15b => Bit64::R15,
            NibbleRegister::R14b => Bit64::R14,
            NibbleRegister::R13b => Bit64::R13,
            NibbleRegister::R12b => Bit64::R12,
            NibbleRegister::R8b => Bit64::R8,
            NibbleRegister::R9b => Bit64::R9,
        }
    }

    pub fn to_32_bit_register(&self) -> Bit32 {
        match self {
            NibbleRegister::AH => Bit32::Eax,
            NibbleRegister::AL => Bit32::Eax,
            NibbleRegister::BH => Bit32::Ecx,
            NibbleRegister::BL => Bit32::Ecx,
            NibbleRegister::CH => Bit32::Edi,
            NibbleRegister::CL => Bit32::Edi,
            NibbleRegister::SPL => Bit32::Edx,
            NibbleRegister::BPL => Bit32::Edx,
            NibbleRegister::DIL => Bit32::Edx,
            NibbleRegister::SIL => Bit32::Edx,
            NibbleRegister::DH => Bit32::Edx,
            NibbleRegister::DL => Bit32::Edx,
            NibbleRegister::R15b => Bit32::R15d,
            NibbleRegister::R8b => Bit32::R8d,
            NibbleRegister::R9b => Bit32::R9d,
            NibbleRegister::R14b => Bit32::R14d,
            NibbleRegister::R13b => Bit32::R13d,
            NibbleRegister::R12b => Bit32::R12d,
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
            NibbleRegister::DIL => "dil",
            NibbleRegister::SIL => "sil",
            NibbleRegister::DH => "dh",
            NibbleRegister::DL => "dl",
            NibbleRegister::R15b => "r15b",
            NibbleRegister::R14b => "r14b",
            NibbleRegister::R13b => "r13b",
            NibbleRegister::R12b => "r12b",
            NibbleRegister::R8b => "r8b",
            NibbleRegister::R9b => "r9b",
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
            Bit16::R15w => "r15w",
            Bit16::R14w => "r14w",
            Bit16::R13w => "r13w",
            Bit16::R12w => "r12w",
            Bit16::R8w => "r8w",
            Bit16::R9w => "r9w",
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
            Bit32::R15d => "r15d",
            Bit32::R8d => "r8d",
            Bit32::R9d => "r9d",
            Bit32::R14d => "r14d",
            Bit32::R13d => "r13d",
            Bit32::R12d => "r12d"
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
            Bit64::R15 => "r15",
            Bit64::R14 => "r14",
            Bit64::R13 => "r13",
            Bit64::R12 => "r12"
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