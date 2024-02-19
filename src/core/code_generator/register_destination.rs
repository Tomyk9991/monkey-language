/// Returns the responsible register according to the provided byte size
/// # Example
///  - byte = 8 => rax
///  - byte = 4 => rax
///  - byte = 2 => ax
///  - byte = 1 => al
#[allow(dead_code)]
pub(crate) fn from_byte_size(byte: usize) -> String {
    match byte {
        8 => "rax",
        4 => "eax",
        2 => "ax",
        1 => "al",
        0 => "eax",
        _ => "undefined byte"
    }.to_string()
}

pub fn word_from_byte_size(byte: usize) -> String {
    match byte {
        8 => "QWORD",
        4 => "DWORD",
        2 => "WORD",
        1 => "BYTE",
        _ => ""
    }.to_string()
}