extern fn printf(format: *string, value: i32): void;

fn mut_ref(x: mut *i32): void {
    *x = *x + 1;
}

let mut a: i32 = 5;
mut_ref(&a);

printf("Value of a after mut_ref: %d\n", a);