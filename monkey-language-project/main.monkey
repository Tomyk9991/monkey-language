module monkey-language-project/std.monkey;

fn main(): i32 {
    for (let a: i32 = 0; a < 5; a = a + 1) {
        if (a == 4) {
            printf("%d", a);
        } else {
            printf("%d\n", a);
        }
    }
    return 0;
}