module monkey-language-project/std.monkey;

fn f1(): f64 { return 13.0_f64; }
fn f2(): f64 { return 5.0_f64; }

fn main(): i32 {
    let addition = (((f1() + f2()) + (f2() + f1())) + (f2() + f2())) + ((f2() + (f2() + f2())) + (f2() + (f1() + f2())));
    printf("value is: %f", addition);
    return 0;
}