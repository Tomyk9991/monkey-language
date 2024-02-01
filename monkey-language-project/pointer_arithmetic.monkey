module monkey-language-project/std.monkey;

let a: f64 = 5.0;
let b: f64 = 3.0;
let c: f64 = 7.0;
let d: f64 = 9.0;

let result = (a == b && c != d && a >= b) || (c <= d && a < b && c > d);

printf("%d", (i32) result);
ExitProcess(0);