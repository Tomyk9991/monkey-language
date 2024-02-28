module monkey-language-project/std.monkey;

fn inc(a: i32): i32 {
    return a + 1;
}

fn main(): i32 {
    printf("%d", inc(inc(1)));
    return 0;
}