module monkey-language-project/std.monkey;

let a: f32 = 1.0;
let c: f64 = (f64) a;

let b: f32 = (f32)(f64)(a + 1.0);

printf("%.2f", (f64) b);

ExitProcess(0);