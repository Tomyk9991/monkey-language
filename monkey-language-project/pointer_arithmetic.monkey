module monkey-language-project/std.monkey;


let a: i32 = 5;
let b: *i32 = &a;
let c: **i32 = &b;
let d: *i32 = *c;

let ref: **i32 = c;
let f: i32 = *d;
let g: i32 = **c;

let format: *string = "Das ist ein Test %d";
printf(format, *b);

ExitProcess(*b);