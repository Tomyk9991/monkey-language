module monkey-language-project/std.monkey;

let format: *string = "The formatted thing is:\n\t%d\n";
let mut reference = format;

reference = "Rofl mofl";

let mut value: i32 = 5;
value = value + 1;

let s = 5;

printf(format, value);
printf(reference, value + 1);

ExitProcess(0);








let format: *string = "Das ist ein Test 0x%p";
printf(format, g);

ExitProcess(*b);


let a: i32 = 5;
let b: *i32 = &a;

let c: i32 = 13;
let d: *i32 = &c;

let addition = (f32) (((1 + 2) + (3 + 4)) + (5 + 6));

ExitProcess(addition);