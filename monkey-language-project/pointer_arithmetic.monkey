module monkey-language-project/std.monkey;


fn fib(n: i32): i32 {
    let condition = n <= 1;

    if (condition) {
        return n;
    }

    return fib(n - 1) + fib(n - 2);
}

fn main(): i32 {
    let fib_result = fib(9);
    printf("%d", fib_result);
    return 0;
}