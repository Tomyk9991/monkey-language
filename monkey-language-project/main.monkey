module monkey-language-project/std.monkey;

fn index(value: i32): i32 {
    return value;
}

fn main(): i32 {
    let mut a: [i32, 5] = [1, 2, 3, 4, 5];
    a = [5, 4, 3, 2, 1];
    let b = a[0];
    printf("%d\n", b);
    return 0;
}