module monkey-language-project/std.monkey;

fn mut_ref(x: mut *i32): void {
    *x = *x + 1;
}

fn main(): i32 {
    let a: i32 = 5;
    mut_ref(&a);
    printf("%d\n", a);
    return 0;
}