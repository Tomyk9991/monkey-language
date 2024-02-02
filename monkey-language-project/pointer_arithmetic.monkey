module monkey-language-project/std.monkey;


fn print_i32(b: i32) {
    printf("%d\n", b);
}

fn main(): i32 {
    let a = 5;
    print_i32(a);
}