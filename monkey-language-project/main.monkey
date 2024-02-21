module monkey-language-project/std.monkey;

fn add(a: i32, b: i32): i32 {
    return a + b;
}

fn main(): i32 {
    let a = add((1 + 2) * (3 + 4), (5 + 6) * (7 + 8));
    printf("%d", a);
    return 0;
}