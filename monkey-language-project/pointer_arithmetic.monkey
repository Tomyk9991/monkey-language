module monkey-language-project/std.monkey;

let a: f64 = 10.2;
let b: f64 = 5.7;

let c = a <= 10.2_f64;
let d = 10.2_f64 <= 5.7_f64;

printf("%d\n", (i32)c);
printf("%d\n", (i32)d);


ExitProcess(0);