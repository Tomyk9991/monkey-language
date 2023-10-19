/// Returns the responsible register according to the provided byte size
/// # Example
///  - byte = 8 => rax
///  - byte = 4 => rax
///  - byte = 2 => ax
///  - byte = 1 => al
pub fn from_byte_size(byte: usize) -> String {
    return format!("{}", match byte {
        8 => "rax",
        4 => "eax",
        2 => "ax",
        1 => "al",
        0 => "eax",
        _ => "undefined byte"
    });
}

pub fn word_from_byte_size(byte: usize) -> String {
    return format!("{}", match byte {
        8 => "QWORD",
        4 => "DWORD",
        _ => ""
    })
}