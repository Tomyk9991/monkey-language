module monkey-language-project/std.monkey;

fn constant_1(): i32 { return 30; }

fn main(): i32 {
    let a: i32 = 5;
    let b = 25 + constant_1();
    let c = *constant_1() + 25;

    let d = b + c;

    let e = constant_1() + constant_1();

    return e;
}