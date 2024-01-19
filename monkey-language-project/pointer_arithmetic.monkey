module monkey-language-project/std.monkey;

let a: f32 = 1.0;
let b: f64 = (f64)(a + 1.0);

printf("%.2f", b);

ExitProcess(0);