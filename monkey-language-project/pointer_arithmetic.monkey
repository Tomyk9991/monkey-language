module monkey-language-project/std.monkey;


let a: f64 = 5.0_f64;
let b: *f64 = &a;
let c: **f64 = &b;

let d = (f32)**c;

printf("%.2f", (f64) d);

ExitProcess(0);