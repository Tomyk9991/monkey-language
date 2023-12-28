module monkey-language-project/std.monkey;

let a: i32 = 5;
let b: i32 = a + 1;


let format: *string = "%d";
printf(format, a);

ExitProcess(b);