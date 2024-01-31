module monkey-language-project/std.monkey;

let a: bool = (5 + 3) * 2 == 16;
let c: bool = 5 < 3;

let first = 10;
let second = 7;

let b: i32 = first & second;
let d: bool = true & true;

printf("%d", (i32)d);
ExitProcess(0);