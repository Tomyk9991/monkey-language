module monkey-language-project/std.monkey;

let a: bool = true;
let b: *bool = &a;

let c: bool = true;
let d: *bool = &c;

let addition = (((*d | *b) | (*b | *d)) | (*b | *b)) | ((*b | (*b | *b)) | (*b | (*d | *b)));

printf("%d", (i32)addition);

ExitProcess(0);