module monkey-language-project/std.monkey;

fn main(): i32 {
    let mut a: [i32, 5] = [1, 2, 3, 4, 5];
    a[0] = 10;

    for (let mut i = 0; i < 5; i = i + 1) {
        let d = a[i];
        printf("Iteration %d", i);
        printf("' = %d'\n", d);
    }

    return 0;
}