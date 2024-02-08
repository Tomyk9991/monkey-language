module monkey-language-project/std.monkey;


fn main(): i32 {
    let a: f64 = 5.0_f64;
    let b: f32 = (f32)(a + 1.0_f64);
    printf("%f", (f64) b);

    return 0;
}