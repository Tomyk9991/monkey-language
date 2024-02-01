module monkey-language-project/std.monkey;

let a: i32 = 5;
let b: i32 = 3;
let c: i32 = 7;
let d: i32 = 9;

let result = (a == b && c != d && a >= b) || (c <= d && a < b && c > d);

printf("%d", (i32) result);

ExitProcess(0);