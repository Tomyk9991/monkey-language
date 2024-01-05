module monkey-language-project/std.monkey;

let a: i32 = 5;
let b: *i32 = &a;

let c: i32 = 13;
let d: *i32 = &c;

let addition = (f32) (((1 + 2) + (3 + 4)) + (5 + 6));

ExitProcess(addition);
