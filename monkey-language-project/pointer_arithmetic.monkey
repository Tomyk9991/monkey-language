module monkey-language-project/std.monkey;

let a: i32 = 5;
let b: *i32 = &a;

let c: i32 = 13;
let d: *i32 = &c;

let addition = (((*d + *b) + (*b + *d)) + (*b + *b)) + ((*b + (*b + *b)) + (*b + (*d + *b)));

printf("%d", addition);

ExitProcess(0);