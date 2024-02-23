module monkey-language-project/std.monkey;

fn main(): i32 {
    for (let a: i32 = 0; a < 5; a = a + 1) {
        printf("%d", a);
    }
    return 0;
}