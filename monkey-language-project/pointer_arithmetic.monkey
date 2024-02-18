module monkey-language-project/std.monkey;


fn f1(): i32 { return 5; }
fn f2(): i32 { return 1; }

fn main(): i32 {
    let a: i32 = f1();
    let b: *i32 = &a;
    let addition = *b + f2();
    printf("%d", addition);
    return 0;
}