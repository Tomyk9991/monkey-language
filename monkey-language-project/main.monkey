module monkey-language-project/std.monkey;

fn f1(): f64 { return 13.0_f64; }
fn f2(): f64 { return 5.0_f64; }

fn f3(): f64 { return 5.0_f64; }
fn f4(): f64 { return 13.0_f64; }
fn f5(): f64 { return 5.0_f64; }
fn f6(): f64 { return 5.0_f64; }
fn f7(): f64 { return 5.0_f64; }
fn f8(): f64 { return 5.0_f64; }
fn f9(): f64 { return 5.0_f64; }
fn f10(): f64 { return 5.0_f64; }
fn f11(): f64 { return 13.0_f64; }
fn f12(): f64 { return 5.0_f64; }

fn main(): i32 {
    let addition = f1() + (f2() + f3());
    printf("%f", addition);
    let r = (((f1() + f2()) + (f3() + f4())) + (f5() + f6())) + ((f7() + (f8() + f9())) + (f10() + (f1() + f12())));
    return 0;
}