module monkey-language-project/std.monkey;

fn inc(a: i32): i32 {
    return a + 1;
}

fn main(): i32 {
    let mut a: i32 = 0;

    for (let i: i32 = 0; i < 5; i = i + 1) {
        for (let j: i32 = 0; j < 5; j = j + 1) {
            a = inc(a);
        }
    }


    printf("%d", a);
    return 0;
}