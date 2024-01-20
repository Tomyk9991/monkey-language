module monkey-language-project/std.monkey;

let b: f64 = 5.0_f64;
let d: f64 = 13.0_f64;
let addition: f32 = ((((f32)d + (f32)b) + ((f32)b + (f32)d)) + ((f32)b + (f32)b)) + (((f32)b + ((f32)b + (f32)b)) + ((f32)b + ((f32)d + (f32)b)));

printf("%f", (f64) addition);

ExitProcess(0);