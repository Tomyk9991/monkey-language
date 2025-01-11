module monkey-language-project/std.monkey;

fn main(): i32 {
    let mut a: [i32, 5] = [1, 2, 3, 4, 5];
    let b = a[0];
    a[0] = 10;
    let c = a[0];

    printf("%d\n", b);
    printf("%d\n", c);
    return 0;
}