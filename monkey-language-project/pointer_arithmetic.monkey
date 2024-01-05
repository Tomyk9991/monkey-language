module monkey-language-project/std.monkey;

let a: i32 = 5;
let b: *i32 = &a;

let c: i32 = 13;
let d: *i32 = &c;

let addition = (f32) (((*d + *b) + (*b + *d)) + (*b + *b)) + ((*b + (*b + *b)) + (*b + (*d + *b)));

ExitProcess(addition);

