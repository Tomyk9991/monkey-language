module monkey-language-project/std.monkey;

fn float_return(): f32 { return 5.0; }
fn float_f64_return(): f64 { return 5.0_f64; }
fn integer_return(): i32 { return 23; }

fn main(): i32 {
    let a: f32 = float_return();
    let b: f64 = float_f64_return();
    let c: i32 = integer_return() + (i32) b;

    return 0;
}