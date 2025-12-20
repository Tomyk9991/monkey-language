use crate::core::code_generator::registers::GeneralPurposeRegister;

#[derive(Debug, Clone)]
pub struct InterimResultOption {
    pub general_purpose_register: GeneralPurposeRegister
}

impl From<&GeneralPurposeRegister> for InterimResultOption {
    fn from(value: &GeneralPurposeRegister) -> Self {
        InterimResultOption {
            general_purpose_register: value.clone(),
        }
    }
}