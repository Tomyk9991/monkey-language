extern fn printf(format: *string, value: i32): void;

fn method_call(): i32 {
    return 42 + 3;
}

let a = 3 + 2;
let b = 10 * a + method_call();

printf("The result is: %d", b);