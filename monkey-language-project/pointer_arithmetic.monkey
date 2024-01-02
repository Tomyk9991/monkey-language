module monkey-language-project/std.monkey;

let a: i32 = 5;
let b: *i32 = &a;

let addition = (*b + *b) + (*b + *b);

ExitProcess(addition);

