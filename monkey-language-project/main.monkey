module monkey-language-project/std.monkey;

fn index(value: i32): i32 {
    return value;
}

fn main(): i32 {
    let a: [f64, 3] = [1.0_f64, 2.0_f64, 3.0_f64];
    let l = index();
    let b = a[1 + index(1)];
    printf("%f\n", b);
    return 0;
}