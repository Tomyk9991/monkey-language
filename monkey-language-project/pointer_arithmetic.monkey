module monkey-language-project/std.monkey;


fn r(): f64 { return 5.0_f64; }

fn main(): i32 {
    let a: f64 = r();
    let b: *f64 = &a;
    let c: **f64 = &b;
    let d: *f64 = *c;

    let ref: **f64 = c;
    let f: f64 = *d;
    let g: f64 = **c;
    let h: *f64 = &r();

    printf("Program output: 0x%p\n", h);
    printf("%f", *h);
    return 0;
}